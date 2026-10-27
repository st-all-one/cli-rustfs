//! Progress bar utilities for transfer operations
//!
//! Provides consistent progress indication for long-running operations
//! like file transfers and sync operations.

use super::OutputConfig;

/// Progress bar wrapper
///
/// Handles progress display based on output configuration.
/// In quiet or JSON mode, progress is suppressed.
#[derive(Debug)]
#[allow(
    dead_code,
    reason = "public compatibility surface kept for upcoming commands"
)]
pub struct ProgressBar {
    config: OutputConfig,
    bar: Option<indicatif::ProgressBar>,
}

#[allow(
    dead_code,
    reason = "public compatibility surface kept for upcoming commands"
)]
impl ProgressBar {
    /// Create a new progress bar with the given total size
    pub(crate) fn new(config: OutputConfig, total: u64) -> Self {
        let bar = if config.quiet || config.json || config.no_progress {
            None
        } else {
            let bar = indicatif::ProgressBar::new(total);
            bar.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap_or_else(|_| indicatif::ProgressStyle::default_bar())
                    .progress_chars("#>-"),
            );
            Some(bar)
        };

        Self { config, bar }
    }

    /// Create a spinner for indeterminate progress
    pub(crate) fn spinner(config: OutputConfig, message: &str) -> Self {
        let bar = if config.quiet || config.json || config.no_progress {
            None
        } else {
            let bar = indicatif::ProgressBar::new_spinner();
            bar.set_style(
                indicatif::ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap_or_else(|_| indicatif::ProgressStyle::default_spinner()),
            );
            bar.set_message(message.to_string());
            bar.enable_steady_tick(std::time::Duration::from_millis(100));
            Some(bar)
        };

        Self { config, bar }
    }

    /// Update progress
    pub(crate) fn set_position(&self, pos: u64) {
        if let Some(bar) = &self.bar {
            bar.set_position(pos);
        }
    }

    /// Increment progress
    pub(crate) fn inc(&self, delta: u64) {
        if let Some(bar) = &self.bar {
            bar.inc(delta);
        }
    }

    /// Set message
    pub(crate) fn set_message(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.set_message(message.to_string());
        }
    }

    /// Finish with a message
    pub(crate) fn finish_with_message(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.finish_with_message(message.to_string());
        }
    }

    /// Finish and clear the progress bar
    pub(crate) fn finish_and_clear(&self) {
        if let Some(bar) = &self.bar {
            bar.finish_and_clear();
        }
    }

    /// Check if progress bar is visible
    pub(crate) fn is_visible(&self) -> bool {
        self.bar.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_quiet_mode() {
        let config = OutputConfig {
            quiet: true,
            ..Default::default()
        };
        let bar = ProgressBar::new(config, 100);
        assert!(!bar.is_visible());
    }

    #[test]
    fn test_progress_bar_json_mode() {
        let config = OutputConfig {
            json: true,
            ..Default::default()
        };
        let bar = ProgressBar::new(config, 100);
        assert!(!bar.is_visible());
    }

    #[test]
    fn test_progress_bar_no_progress() {
        let config = OutputConfig {
            no_progress: true,
            ..Default::default()
        };
        let bar = ProgressBar::new(config, 100);
        assert!(!bar.is_visible());
    }

    #[test]
    fn test_progress_bar_normal() {
        let config = OutputConfig::default();
        let bar = ProgressBar::new(config, 100);
        assert!(bar.is_visible());
    }
}
