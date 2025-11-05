mod events;
mod feedback;
mod state;
mod stats;
mod ui;

use eframe::egui;
use events::{EventListener, GameEvent};
use feedback::FeedSystem;
use state::{CounterStrafeState, StrafeKey};
use stats::Stats;
use std::time::Instant;

const WINDOW_TITLE: &str = "CS2 Counter-Strafe Trainer";

struct CS2TrainerApp {
    event_listener: EventListener,
    state: CounterStrafeState,
    feed: FeedSystem,
    stats: Stats,
    should_quit: bool,
}

impl CS2TrainerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let event_listener = EventListener::start()
            .expect("Failed to start event listener. Do you have permission to read keyboard events?");

        Self {
            event_listener,
            state: CounterStrafeState::new(),
            feed: FeedSystem::new(),
            stats: Stats::new(),
            should_quit: false,
        }
    }

    fn process_events(&mut self) {
        let now = Instant::now();
        let events = self.event_listener.drain_events();

        for event in events {
            match event {
                GameEvent::KeyAPress => {
                    if let Some(result) = self.state.on_key_press(StrafeKey::A, now) {
                        self.handle_completion(result);
                    }
                }
                GameEvent::KeyARelease => {
                    if let Some(result) = self.state.on_key_release(StrafeKey::A, now) {
                        self.handle_completion(result);
                    }
                }
                GameEvent::KeyDPress => {
                    if let Some(result) = self.state.on_key_press(StrafeKey::D, now) {
                        self.handle_completion(result);
                    }
                }
                GameEvent::KeyDRelease => {
                    if let Some(result) = self.state.on_key_release(StrafeKey::D, now) {
                        self.handle_completion(result);
                    }
                }
                GameEvent::SpacePress => {
                    // Optional: handle shooting
                }
                GameEvent::EscapePress => {
                    self.should_quit = true;
                }
            }
        }

        // Check for timeout
        self.state.check_timeout(now);

        // Cleanup expired feed entries
        self.feed.cleanup(now);
    }

    fn handle_completion(&mut self, result: state::CompletionResult) {
        // Record stats
        self.stats.record(result.quality);

        // Add to feed
        if let Some(error_msg) = result.error_message {
            self.feed.add_failed(&error_msg);
        } else {
            match result.quality {
                state::Quality::Perfect => self.feed.add_perfect(result.hold_time),
                state::Quality::Good => self.feed.add_good(result.hold_time),
                state::Quality::Failed => {
                    // Should not happen without error message, but handle it
                    self.feed.add_failed(&format!("Failed {:.0}ms", result.hold_time * 1000.0));
                }
            }
        }
    }
}

impl eframe::App for CS2TrainerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process keyboard events
        self.process_events();

        // Render UI
        ui::render_ui(ctx, &self.state, &self.feed, &self.stats);

        // Handle quit
        if self.should_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Request continuous repaint for smooth animations and timer updates
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    // Check permissions on Linux
    #[cfg(target_os = "linux")]
    check_permissions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([ui::WINDOW_WIDTH, ui::WINDOW_HEIGHT])
            .with_min_inner_size([ui::WINDOW_WIDTH, ui::WINDOW_HEIGHT])
            .with_max_inner_size([ui::WINDOW_WIDTH, ui::WINDOW_HEIGHT])
            .with_decorations(false)
            .with_resizable(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|cc| Ok(Box::new(CS2TrainerApp::new(cc)))),
    )
}

#[cfg(target_os = "linux")]
fn check_permissions() {
    use std::os::unix::fs::PermissionsExt;

    // Try to check /dev/input/event0 permissions
    if let Ok(metadata) = std::fs::metadata("/dev/input/event0") {
        if metadata.permissions().mode() & 0o4 == 0 {
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            eprintln!("WARNING: May need elevated permissions for keyboard access");
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            eprintln!("");
            eprintln!("Option 1: Add user to input group (recommended):");
            eprintln!("  sudo usermod -a -G input $USER");
            eprintln!("  (then log out and back in)");
            eprintln!("");
            eprintln!("Option 2: Run with sudo:");
            eprintln!("  sudo ./cs2-counter-strafe-trainer");
            eprintln!("");
        }
    }
}
