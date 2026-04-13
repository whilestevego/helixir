use std::path::PathBuf;
use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent};
use tokio::sync::mpsc;

pub enum AppEvent {
    Key(KeyEvent),
    FileChanged(PathBuf),
    Tick,
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<AppEvent>,
}

impl EventHandler {
    pub fn new(exercises_dir: PathBuf) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Crossterm key events + tick
        let tx_keys = tx.clone();
        tokio::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(250)).unwrap_or(false) {
                    if let Ok(Event::Key(key)) = event::read()
                        && tx_keys.send(AppEvent::Key(key)).is_err()
                    {
                        break;
                    }
                } else if tx_keys.send(AppEvent::Tick).is_err() {
                    break;
                }
            }
        });

        // File watcher
        let tx_files = tx;
        tokio::spawn(async move {
            use notify_debouncer_mini::{DebouncedEventKind, new_debouncer};

            let (notify_tx, mut notify_rx) = mpsc::unbounded_channel();

            let mut debouncer = new_debouncer(
                Duration::from_millis(300),
                move |events: Result<Vec<notify_debouncer_mini::DebouncedEvent>, notify::Error>| {
                    if let Ok(events) = events {
                        for event in events {
                            if event.kind == DebouncedEventKind::Any {
                                let _ = notify_tx.send(event.path);
                            }
                        }
                    }
                },
            )
            .expect("failed to create file watcher");

            debouncer
                .watcher()
                .watch(&exercises_dir, notify::RecursiveMode::Recursive)
                .expect("failed to watch exercises directory");

            // Keep debouncer alive and forward events
            while let Some(path) = notify_rx.recv().await {
                if path.extension().is_some_and(|e| e == "hxt")
                    && tx_files.send(AppEvent::FileChanged(path)).is_err()
                {
                    break;
                }
            }
        });

        EventHandler { rx }
    }

    pub async fn next(&mut self) -> anyhow::Result<AppEvent> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("event channel closed"))
    }
}
