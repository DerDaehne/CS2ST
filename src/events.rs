use rdev::{Event, EventType, Key};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    KeyAPress,
    KeyARelease,
    KeyDPress,
    KeyDRelease,
    SpacePress,
    EscapePress,
}

pub struct EventListener {
    receiver: Receiver<GameEvent>,
}

impl EventListener {
    /// Start listening for keyboard events in a background thread
    pub fn start() -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = channel();

        // Spawn background thread for rdev event listening
        thread::spawn(move || {
            if let Err(e) = listen_events(tx) {
                eprintln!("Error in event listener: {}", e);
            }
        });

        Ok(Self { receiver: rx })
    }

    /// Try to receive next event (non-blocking)
    pub fn try_recv(&self) -> Option<GameEvent> {
        self.receiver.try_recv().ok()
    }

    /// Get all pending events (non-blocking)
    pub fn drain_events(&self) -> Vec<GameEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.receiver.try_recv() {
            events.push(event);
        }
        events
    }
}

/// Background event listening function
fn listen_events(tx: Sender<GameEvent>) -> Result<(), Box<dyn std::error::Error>> {
    // Track key states to filter key repeats
    let mut a_pressed = false;
    let mut d_pressed = false;

    rdev::listen(move |event: Event| {
        let game_event = match event.event_type {
            EventType::KeyPress(Key::KeyA) | EventType::KeyPress(Key::Alt) => {
                if !a_pressed {
                    a_pressed = true;
                    Some(GameEvent::KeyAPress)
                } else {
                    None // Filter key repeat
                }
            }
            EventType::KeyRelease(Key::KeyA) | EventType::KeyRelease(Key::Alt) => {
                if a_pressed {
                    a_pressed = false;
                    Some(GameEvent::KeyARelease)
                } else {
                    None
                }
            }
            EventType::KeyPress(Key::KeyD) => {
                if !d_pressed {
                    d_pressed = true;
                    Some(GameEvent::KeyDPress)
                } else {
                    None // Filter key repeat
                }
            }
            EventType::KeyRelease(Key::KeyD) => {
                if d_pressed {
                    d_pressed = false;
                    Some(GameEvent::KeyDRelease)
                } else {
                    None
                }
            }
            EventType::KeyPress(Key::Space) => Some(GameEvent::SpacePress),
            EventType::KeyPress(Key::Escape) => Some(GameEvent::EscapePress),
            _ => None,
        };

        if let Some(evt) = game_event {
            // Send event through channel (non-blocking)
            let _ = tx.send(evt);
        }
    })
    .map_err(|e| format!("Event listening error: {:?}", e).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_event_equality() {
        assert_eq!(GameEvent::KeyAPress, GameEvent::KeyAPress);
        assert_ne!(GameEvent::KeyAPress, GameEvent::KeyDPress);
    }
}
