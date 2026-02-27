use std::time::{Duration, Instant};

#[derive(Clone)]
pub enum NotificationPriority {
    Low,  // Info messages, yields to Working status
    High, // Errors/warnings, overrides Working status
}

#[derive(Clone)]
pub struct AppNotification {
    pub message: String,
    pub priority: NotificationPriority,
    pub duration: Duration,
    pub created_at: Instant,
}

impl AppNotification {
    pub fn new(message: impl Into<String>, priority: NotificationPriority) -> Self {
        Self {
            message: message.into(),
            priority,
            duration: Duration::from_secs(1),
            created_at: Instant::now(),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    /// Returns a value from 1.0 (fully visible) to 0.0 (fully faded).
    pub fn fade_ratio(&self) -> f32 {
        let fade_duration = Duration::from_millis(300);
        let elapsed = self.created_at.elapsed();

        if self.duration <= fade_duration {
            // Duration too short for a fade window; fade over the entire duration
            1.0 - (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
        } else {
            let fade_start = self.duration - fade_duration;
            if elapsed < fade_start {
                1.0
            } else {
                let fade_elapsed = elapsed - fade_start;
                1.0 - (fade_elapsed.as_secs_f32() / fade_duration.as_secs_f32()).clamp(0.0, 1.0)
            }
        }
    }
}
