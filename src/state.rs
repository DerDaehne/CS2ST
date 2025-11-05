use std::time::Instant;

// Timing constants (DO NOT CHANGE!)
pub const OPTIMAL_HOLD_TIME: f32 = 0.080;      // 80ms
pub const MIN_HOLD_TIME: f32 = 0.060;          // 60ms
pub const MAX_HOLD_TIME: f32 = 0.120;          // 120ms
pub const PERFECT_TOLERANCE: f32 = 0.015;      // ±15ms from optimal
pub const TIMEOUT_NO_COUNTER: f32 = 0.180;     // 180ms timeout if no counter-key

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrafeKey {
    A,
    D,
}

impl StrafeKey {
    pub fn opposite(&self) -> Self {
        match self {
            StrafeKey::A => StrafeKey::D,
            StrafeKey::D => StrafeKey::A,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            StrafeKey::A => 'A',
            StrafeKey::D => 'D',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    Perfect,  // Within 65-95ms (80ms ±15ms)
    Good,     // 60-120ms
    Failed,   // <60ms or >120ms
}

impl Quality {
    pub fn symbol(&self) -> &'static str {
        match self {
            Quality::Perfect => "★",
            Quality::Good => "●",
            Quality::Failed => "✕",
        }
    }
}

#[derive(Debug, Clone)]
pub enum CounterStrafeState {
    Idle,
    Strafing {
        key: StrafeKey,
        start_time: Instant,
    },
    Released {
        original_key: StrafeKey,
        release_time: Instant,
    },
    CounterStrafing {
        original_key: StrafeKey,
        counter_key: StrafeKey,
        start_time: Instant,
    },
    Completed {
        hold_time: f32,
        quality: Quality,
        error_message: Option<String>,
    },
}

impl CounterStrafeState {
    pub fn new() -> Self {
        CounterStrafeState::Idle
    }

    /// Handle key press event
    pub fn on_key_press(&mut self, key: StrafeKey, now: Instant) -> Option<CompletionResult> {
        match self {
            CounterStrafeState::Idle => {
                *self = CounterStrafeState::Strafing {
                    key,
                    start_time: now,
                };
                None
            }
            CounterStrafeState::Strafing { key: current_key, .. } => {
                // Pressing same key again = key repeat, ignore
                if *current_key == key {
                    None
                } else {
                    // Pressing opposite key before releasing = error
                    *self = CounterStrafeState::Completed {
                        hold_time: 0.0,
                        quality: Quality::Failed,
                        error_message: Some("Both keys pressed".to_string()),
                    };
                    Some(CompletionResult {
                        hold_time: 0.0,
                        quality: Quality::Failed,
                        error_message: Some("Both keys pressed".to_string()),
                    })
                }
            }
            CounterStrafeState::Released { original_key, release_time: _ } => {
                if key == *original_key {
                    // Pressing same key again = restart
                    *self = CounterStrafeState::Strafing {
                        key,
                        start_time: now,
                    };
                    None
                } else {
                    // Pressing opposite key = start counter-strafing
                    *self = CounterStrafeState::CounterStrafing {
                        original_key: *original_key,
                        counter_key: key,
                        start_time: now,
                    };
                    None
                }
            }
            CounterStrafeState::CounterStrafing { .. } => {
                // Key repeat, ignore
                None
            }
            CounterStrafeState::Completed { .. } => {
                // Start new strafe
                *self = CounterStrafeState::Strafing {
                    key,
                    start_time: now,
                };
                None
            }
        }
    }

    /// Handle key release event
    pub fn on_key_release(&mut self, key: StrafeKey, now: Instant) -> Option<CompletionResult> {
        match self {
            CounterStrafeState::Idle => None,
            CounterStrafeState::Strafing { key: current_key, .. } => {
                if *current_key == key {
                    *self = CounterStrafeState::Released {
                        original_key: key,
                        release_time: now,
                    };
                }
                None
            }
            CounterStrafeState::Released { .. } => None,
            CounterStrafeState::CounterStrafing {
                original_key: _,
                counter_key,
                start_time,
            } => {
                if *counter_key == key {
                    // Calculate hold time
                    let hold_time = now.duration_since(*start_time).as_secs_f32();
                    let quality = evaluate_hold_time(hold_time);

                    let error_message = if hold_time < MIN_HOLD_TIME {
                        Some(format!("Too fast {:.0}ms", hold_time * 1000.0))
                    } else if hold_time > MAX_HOLD_TIME {
                        Some(format!("Too slow {:.0}ms", hold_time * 1000.0))
                    } else {
                        None
                    };

                    *self = CounterStrafeState::Completed {
                        hold_time,
                        quality,
                        error_message: error_message.clone(),
                    };

                    Some(CompletionResult {
                        hold_time,
                        quality,
                        error_message,
                    })
                } else {
                    None
                }
            }
            CounterStrafeState::Completed { .. } => None,
        }
    }

    /// Check for timeout (180ms without counter-key)
    pub fn check_timeout(&mut self, now: Instant) -> bool {
        if let CounterStrafeState::Released { release_time, .. } = self {
            let elapsed = now.duration_since(*release_time).as_secs_f32();
            if elapsed >= TIMEOUT_NO_COUNTER {
                *self = CounterStrafeState::Idle;
                return true;
            }
        }
        false
    }

    /// Reset to idle
    pub fn reset(&mut self) {
        *self = CounterStrafeState::Idle;
    }

    /// Get current hold time if counter-strafing
    pub fn get_current_hold_time(&self, now: Instant) -> Option<f32> {
        if let CounterStrafeState::CounterStrafing { start_time, .. } = self {
            Some(now.duration_since(*start_time).as_secs_f32())
        } else {
            None
        }
    }

    /// Get display info for UI
    pub fn get_display_info(&self) -> StateDisplayInfo {
        match self {
            CounterStrafeState::Idle => StateDisplayInfo {
                main_text: "READY".to_string(),
                sub_text: Some("Press A or D".to_string()),
                show_target: false,
            },
            CounterStrafeState::Strafing { key, .. } => StateDisplayInfo {
                main_text: "RELEASE".to_string(),
                sub_text: Some(format!("Release {}", key.as_char())),
                show_target: false,
            },
            CounterStrafeState::Released { original_key, .. } => StateDisplayInfo {
                main_text: "COUNTER".to_string(),
                sub_text: Some(format!("Press {}", original_key.opposite().as_char())),
                show_target: false,
            },
            CounterStrafeState::CounterStrafing { .. } => StateDisplayInfo {
                main_text: "".to_string(),  // Will show timer instead
                sub_text: None,
                show_target: true,
            },
            CounterStrafeState::Completed { .. } => StateDisplayInfo {
                main_text: "READY".to_string(),
                sub_text: Some("Press A or D".to_string()),
                show_target: false,
            },
        }
    }
}

pub struct StateDisplayInfo {
    pub main_text: String,
    pub sub_text: Option<String>,
    pub show_target: bool,
}

pub struct CompletionResult {
    pub hold_time: f32,
    pub quality: Quality,
    pub error_message: Option<String>,
}

/// Evaluate hold time quality
pub fn evaluate_hold_time(hold_time: f32) -> Quality {
    if hold_time < MIN_HOLD_TIME || hold_time > MAX_HOLD_TIME {
        Quality::Failed
    } else if (hold_time - OPTIMAL_HOLD_TIME).abs() <= PERFECT_TOLERANCE {
        Quality::Perfect
    } else {
        Quality::Good
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_timing() {
        assert_eq!(evaluate_hold_time(0.080), Quality::Perfect);
        assert_eq!(evaluate_hold_time(0.066), Quality::Perfect);  // 66ms: within tolerance
        assert_eq!(evaluate_hold_time(0.094), Quality::Perfect);  // 94ms: within tolerance
        assert_eq!(evaluate_hold_time(0.070), Quality::Perfect);  // 70ms: clearly within
        assert_eq!(evaluate_hold_time(0.090), Quality::Perfect);  // 90ms: clearly within
    }

    #[test]
    fn test_good_timing() {
        assert_eq!(evaluate_hold_time(0.060), Quality::Good);   // 60ms: at min boundary
        assert_eq!(evaluate_hold_time(0.065), Quality::Good);   // 65ms: boundary case
        assert_eq!(evaluate_hold_time(0.095), Quality::Good);   // 95ms: boundary case
        assert_eq!(evaluate_hold_time(0.100), Quality::Good);   // 100ms: good but not perfect
        assert_eq!(evaluate_hold_time(0.120), Quality::Good);   // 120ms: at max boundary
    }

    #[test]
    fn test_failed_timing() {
        assert_eq!(evaluate_hold_time(0.050), Quality::Failed);
        assert_eq!(evaluate_hold_time(0.059), Quality::Failed);
        assert_eq!(evaluate_hold_time(0.121), Quality::Failed);
        assert_eq!(evaluate_hold_time(0.200), Quality::Failed);
    }

    #[test]
    fn test_state_transitions() {
        let mut state = CounterStrafeState::new();
        let now = Instant::now();

        // Idle -> Strafing
        state.on_key_press(StrafeKey::A, now);
        assert!(matches!(state, CounterStrafeState::Strafing { .. }));

        // Strafing -> Released
        state.on_key_release(StrafeKey::A, now);
        assert!(matches!(state, CounterStrafeState::Released { .. }));

        // Released -> CounterStrafing
        state.on_key_press(StrafeKey::D, now);
        assert!(matches!(state, CounterStrafeState::CounterStrafing { .. }));
    }

    #[test]
    fn test_both_keys_pressed() {
        let mut state = CounterStrafeState::new();
        let now = Instant::now();

        state.on_key_press(StrafeKey::A, now);
        let result = state.on_key_press(StrafeKey::D, now);

        assert!(result.is_some());
        assert_eq!(result.unwrap().quality, Quality::Failed);
        assert!(matches!(state, CounterStrafeState::Completed { .. }));
    }
}
