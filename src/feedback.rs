use std::time::Instant;
use crate::state::Quality;

const MAX_FEED_ENTRIES: usize = 5;
const VISIBLE_DURATION: f32 = 3.0;  // 3 seconds visible
const FADE_DURATION: f32 = 1.0;     // 1 second fade

#[derive(Debug, Clone)]
pub struct FeedEntry {
    pub message: String,
    pub symbol: String,
    pub quality: Quality,
    pub timestamp: Instant,
}

impl FeedEntry {
    pub fn new(message: String, quality: Quality) -> Self {
        Self {
            symbol: quality.symbol().to_string(),
            message,
            quality,
            timestamp: Instant::now(),
        }
    }

    /// Get opacity for fading effect (0.0 to 1.0)
    pub fn get_opacity(&self, now: Instant) -> f32 {
        let elapsed = now.duration_since(self.timestamp).as_secs_f32();

        if elapsed < VISIBLE_DURATION {
            1.0
        } else if elapsed < VISIBLE_DURATION + FADE_DURATION {
            // Fade out linearly
            let fade_progress = (elapsed - VISIBLE_DURATION) / FADE_DURATION;
            1.0 - fade_progress
        } else {
            0.0
        }
    }

    /// Check if entry should be removed
    pub fn is_expired(&self, now: Instant) -> bool {
        let elapsed = now.duration_since(self.timestamp).as_secs_f32();
        elapsed >= VISIBLE_DURATION + FADE_DURATION
    }
}

#[derive(Debug, Clone)]
pub struct FeedSystem {
    entries: Vec<FeedEntry>,
}

impl FeedSystem {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_FEED_ENTRIES),
        }
    }

    /// Add a new feed entry
    pub fn add(&mut self, message: String, quality: Quality) {
        let entry = FeedEntry::new(message, quality);

        // Insert at beginning (newest first)
        self.entries.insert(0, entry);

        // Limit to MAX_FEED_ENTRIES
        if self.entries.len() > MAX_FEED_ENTRIES {
            self.entries.truncate(MAX_FEED_ENTRIES);
        }
    }

    /// Add a perfect attempt
    pub fn add_perfect(&mut self, hold_time: f32) {
        let message = format!("PERFECT {:.0}ms", hold_time * 1000.0);
        self.add(message, Quality::Perfect);
    }

    /// Add a good attempt
    pub fn add_good(&mut self, hold_time: f32) {
        let message = format!("Good {:.0}ms", hold_time * 1000.0);
        self.add(message, Quality::Good);
    }

    /// Add a failed attempt with error message
    pub fn add_failed(&mut self, error_message: &str) {
        self.add(error_message.to_string(), Quality::Failed);
    }

    /// Clean up expired entries
    pub fn cleanup(&mut self, now: Instant) {
        self.entries.retain(|entry| !entry.is_expired(now));
    }

    /// Get all visible entries (newest first)
    pub fn get_visible_entries(&self, now: Instant) -> Vec<&FeedEntry> {
        self.entries
            .iter()
            .filter(|entry| !entry.is_expired(now))
            .collect()
    }

    /// Get entries with opacity for rendering
    pub fn get_entries_with_opacity(&self, now: Instant) -> Vec<(&FeedEntry, f32)> {
        self.entries
            .iter()
            .filter(|entry| !entry.is_expired(now))
            .map(|entry| (entry, entry.get_opacity(now)))
            .collect()
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for FeedSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_add_entries() {
        let mut feed = FeedSystem::new();
        feed.add_perfect(0.080);
        feed.add_good(0.100);
        feed.add_failed("Too fast 50ms");

        assert_eq!(feed.entries.len(), 3);
    }

    #[test]
    fn test_max_entries() {
        let mut feed = FeedSystem::new();
        for i in 0..10 {
            feed.add_perfect(0.080);
        }

        assert_eq!(feed.entries.len(), MAX_FEED_ENTRIES);
    }

    #[test]
    fn test_opacity() {
        let entry = FeedEntry::new("Test".to_string(), Quality::Perfect);
        let now = Instant::now();

        // Should be fully visible immediately
        assert_eq!(entry.get_opacity(now), 1.0);
    }

    #[test]
    fn test_cleanup() {
        let mut feed = FeedSystem::new();
        feed.add_perfect(0.080);

        let now = Instant::now();
        feed.cleanup(now);

        // Should still have the entry
        assert_eq!(feed.entries.len(), 1);
    }
}
