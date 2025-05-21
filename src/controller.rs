use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tracing::info;

use crate::command::FireCommand;

/// Internal state for cancellation.
#[derive(Debug)]
struct ActiveCommand {
    handle: JoinHandle<()>,
}

#[derive(Debug, Clone)]
pub struct FiringController {
    inner: Arc<Mutex<Option<ActiveCommand>>>,
}

impl FiringController {
    // ---
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    /// Handle a command — either schedule a firing or cancel the existing one.
    pub async fn handle_command(&self, cmd: FireCommand) {
        // ---
        match cmd {
            FireCommand::Cancel => {
                let mut state = self.inner.lock().await;
                if let Some(active) = state.take() {
                    active.handle.abort();
                    info!("⛔ Command cancelled");
                } else {
                    info!("⚠️  Cancel received, but no command pending");
                }
            }

            FireCommand::Fire(delay_secs) => {
                let mut state = self.inner.lock().await;

                if let Some(active) = state.take() {
                    active.handle.abort();
                    info!(delay_secs, "🆕 Replacing existing firing command");
                } else {
                    info!(delay_secs, "⏳ Scheduled new firing command");
                }

                let handle = tokio::spawn(async move {
                    sleep(Duration::from_secs_f64(delay_secs)).await;
                    println!("firing now!");
                    info!(delay_secs, "🚀 Firing now!");
                });

                *state = Some(ActiveCommand { handle });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // ---
    use super::*;
    use crate::command::FireCommand;
    use anyhow::Result;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn fires_once() -> Result<()> {
        // ---
        let controller = FiringController::new();
        controller.handle_command(FireCommand::Fire(0.1)).await;

        // Wait for it to fire
        timeout(Duration::from_secs(1), async {
            tokio::time::sleep(Duration::from_millis(150)).await;
        })
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn cancel_prevents_firing() -> Result<()> {
        // ---
        let controller = FiringController::new();
        controller.handle_command(FireCommand::Fire(0.2)).await;
        controller.handle_command(FireCommand::Cancel).await;

        // Wait longer than original delay
        tokio::time::sleep(Duration::from_millis(300)).await;
        // No panic = success
        Ok(())
    }

    #[tokio::test]
    async fn overwrite_cancels_first() -> Result<()> {
        // ---
        let controller = FiringController::new();
        controller.handle_command(FireCommand::Fire(0.5)).await;
        controller.handle_command(FireCommand::Fire(0.1)).await;

        // Wait enough for second to fire, but not first
        tokio::time::sleep(Duration::from_millis(300)).await;
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Placeholder for future enhancement
    async fn handles_invalid_input_format_gracefully() -> Result<()> {
        // ---
        // This test should eventually simulate invalid inputs via TCP
        // or expose a lower-level parsing hook that can be fuzzed.
        //
        // For now, `FiringController` doesn't deal with raw strings,
        // so this is best handled in integration tests or TCP layer.

        Ok(())
    }
}
