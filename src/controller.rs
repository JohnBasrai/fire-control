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

#[derive(Clone)]
pub struct FiringController {
    inner: Arc<Mutex<Option<ActiveCommand>>>,
    fire_action: Arc<dyn Fn() + Send + Sync>,
}

impl FiringController {
    // ---
    pub fn new() -> Self {
        // ---
        fn default_fire() {
            println!("firing now!");
        }

        Self::with_action(default_fire)
    }

    pub fn with_action<F>(fire_fn: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(Mutex::new(None)),
            fire_action: Arc::new(fire_fn),
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

                let fire_action = self.fire_action.clone(); // ownership passed to spawn

                let handle = tokio::spawn(async move {
                    sleep(Duration::from_secs_f64(delay_secs)).await;
                    println!("firing now!");
                    fire_action(); // actual call
                    info!(delay_secs, "🚀 Firing now!");
                });

                *state = Some(ActiveCommand { handle });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::FireCommand;
    use anyhow::{ensure, Result};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::time::Duration;


    fn counter_hook() -> (Arc<AtomicUsize>, impl Fn() + Send + Sync + 'static) {
        // ---
        let count = Arc::new(AtomicUsize::new(0));
        let cloned = count.clone();
        let hook = move || {
            cloned.fetch_add(1, Ordering::SeqCst);
        };
        (count, hook)
    }

    #[tokio::test]
    async fn fires_once() -> Result<()> {
        // ---
        let (count, hook) = counter_hook();
        let controller = FiringController::with_action(hook);

        controller.handle_command(FireCommand::Fire(0.05)).await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        ensure!(count.load(Ordering::SeqCst) == 1, "Expected 1 firing");
        Ok(())
    }

    #[tokio::test]
    async fn cancel_prevents_firing() -> Result<()> {
        // ---
        let (count, hook) = counter_hook();
        let controller = FiringController::with_action(hook);

        controller.handle_command(FireCommand::Fire(0.2)).await;
        controller.handle_command(FireCommand::Cancel).await;
        tokio::time::sleep(Duration::from_millis(300)).await;

        ensure!(count.load(Ordering::SeqCst) == 0, "Expected no firing");
        Ok(())
    }

    #[tokio::test]
    async fn overwrite_cancels_first() -> Result<()> {
        // ---
        let (count, hook) = counter_hook();
        let controller = FiringController::with_action(hook);

        controller.handle_command(FireCommand::Fire(0.5)).await;
        controller.handle_command(FireCommand::Fire(0.1)).await;
        tokio::time::sleep(Duration::from_millis(300)).await;

        ensure!(
            count.load(Ordering::SeqCst) == 1,
            "Expected only one firing (latest)"
        );
        Ok(())
    }
}
