use crate::state::Quality;

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub total_attempts: u32,
    pub perfect_count: u32,
    pub good_count: u32,
    pub failed_count: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a completed counter-strafe attempt
    pub fn record(&mut self, quality: Quality) {
        self.total_attempts += 1;
        match quality {
            Quality::Perfect => self.perfect_count += 1,
            Quality::Good => self.good_count += 1,
            Quality::Failed => self.failed_count += 1,
        }
    }

    /// Get perfect percentage
    pub fn perfect_percentage(&self) -> f32 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.perfect_count as f32 / self.total_attempts as f32) * 100.0
        }
    }

    /// Get good percentage
    pub fn good_percentage(&self) -> f32 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.good_count as f32 / self.total_attempts as f32) * 100.0
        }
    }

    /// Get failed percentage
    pub fn failed_percentage(&self) -> f32 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.failed_count as f32 / self.total_attempts as f32) * 100.0
        }
    }

    /// Reset all statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_stats() {
        let stats = Stats::new();
        assert_eq!(stats.total_attempts, 0);
        assert_eq!(stats.perfect_percentage(), 0.0);
    }

    #[test]
    fn test_record_attempts() {
        let mut stats = Stats::new();
        stats.record(Quality::Perfect);
        stats.record(Quality::Perfect);
        stats.record(Quality::Good);
        stats.record(Quality::Failed);

        assert_eq!(stats.total_attempts, 4);
        assert_eq!(stats.perfect_count, 2);
        assert_eq!(stats.good_count, 1);
        assert_eq!(stats.failed_count, 1);
        assert_eq!(stats.perfect_percentage(), 50.0);
    }

    #[test]
    fn test_reset() {
        let mut stats = Stats::new();
        stats.record(Quality::Perfect);
        stats.record(Quality::Good);
        stats.reset();

        assert_eq!(stats.total_attempts, 0);
        assert_eq!(stats.perfect_count, 0);
    }
}
