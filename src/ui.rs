use egui::{Color32, RichText, Stroke, Frame, Rounding};
use crate::feedback::FeedSystem;
use crate::state::{CounterStrafeState, Quality, OPTIMAL_HOLD_TIME};
use crate::stats::Stats;
use std::time::Instant;

// Window dimensions (initial size, will adapt to content)
pub const WINDOW_WIDTH: f32 = 460.0;
pub const WINDOW_HEIGHT: f32 = 480.0;
const PADDING: f32 = 20.0;
const SPACING: f32 = 15.0;

// Modern color scheme with vibrant accents
pub const BG_COLOR: Color32 = Color32::from_rgba_premultiplied(10, 12, 20, 220);
pub const CARD_BG: Color32 = Color32::from_rgba_premultiplied(18, 22, 32, 235);
pub const TEXT_COLOR: Color32 = Color32::from_rgb(245, 248, 255);
pub const ACCENT_COLOR: Color32 = Color32::from_rgb(88, 166, 255);    // Modern blue
pub const GOOD_COLOR: Color32 = Color32::from_rgb(52, 211, 153);       // Emerald
pub const BAD_COLOR: Color32 = Color32::from_rgb(248, 113, 113);       // Warm red
pub const WARNING_COLOR: Color32 = Color32::from_rgb(251, 191, 36);    // Amber
pub const NEUTRAL_COLOR: Color32 = Color32::from_rgb(148, 163, 184);   // Slate

// Font sizes
pub const HUGE_FONT: f32 = 56.0;
pub const BIG_FONT: f32 = 28.0;
pub const NORMAL_FONT: f32 = 18.0;
pub const SMALL_FONT: f32 = 14.0;

pub fn render_ui(
    ctx: &egui::Context,
    state: &CounterStrafeState,
    feed: &FeedSystem,
    stats: &Stats,
) {
    let now = Instant::now();

    egui::CentralPanel::default()
        .frame(Frame::none().fill(BG_COLOR).inner_margin(PADDING))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(SPACING);

                // Main display card
                render_main_display(ui, state, now);

                ui.add_space(SPACING);

                // Feed card
                render_feed_card(ui, feed, now);

                ui.add_space(SPACING);

                // Stats bar
                render_stats_bar(ui, stats);

                ui.add_space(SPACING);

                // Controls hint
                render_controls_hint(ui);

                ui.add_space(SPACING);
            });
        });
}

fn render_main_display(ui: &mut egui::Ui, state: &CounterStrafeState, now: Instant) {
    let available_width = ui.available_width();

    let card_frame = Frame::none()
        .fill(CARD_BG)
        .rounding(Rounding::same(12.0))
        .inner_margin(20.0)
        .shadow(egui::epaint::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 20.0,
            spread: 0.0,
            color: Color32::from_black_alpha(100),
        })
        .stroke(Stroke::new(1.5, Color32::from_rgba_premultiplied(88, 166, 255, 40)));

    card_frame.show(ui, |ui| {
        ui.set_min_height(140.0);
        ui.set_width(available_width);

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);

            let display_info = state.get_display_info();

            if display_info.show_target {
                // Show current hold time with symbol
                if let Some(hold_time) = state.get_current_hold_time(now) {
                    let hold_time_ms = (hold_time * 1000.0).round() as i32;

                    // Determine color and symbol based on timing
                    let (color, symbol) = if hold_time < 0.060 {
                        (BAD_COLOR, "⚡")
                    } else if hold_time > 0.120 {
                        (BAD_COLOR, "⏱")
                    } else if (hold_time - OPTIMAL_HOLD_TIME).abs() <= 0.015 {
                        (GOOD_COLOR, "★")
                    } else {
                        (WARNING_COLOR, "●")
                    };

                    let text = format!("{}{} ms", hold_time_ms, symbol);
                    ui.label(
                        RichText::new(text)
                            .color(color)
                            .size(HUGE_FONT)
                            .strong()
                    );

                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("🎯 TARGET: 80ms")
                            .color(NEUTRAL_COLOR)
                            .size(NORMAL_FONT)
                    );
                }
            } else {
                // Show status text
                ui.label(
                    RichText::new(&display_info.main_text)
                        .color(ACCENT_COLOR)
                        .size(BIG_FONT)
                        .strong()
                );

                if let Some(sub_text) = &display_info.sub_text {
                    ui.add_space(5.0);
                    ui.label(
                        RichText::new(sub_text)
                            .color(NEUTRAL_COLOR)
                            .size(NORMAL_FONT)
                    );
                }
            }
        });
    });
}

fn render_feed_card(ui: &mut egui::Ui, feed: &FeedSystem, now: Instant) {
    let available_width = ui.available_width();

    let card_frame = Frame::none()
        .fill(CARD_BG)
        .rounding(Rounding::same(12.0))
        .inner_margin(15.0)
        .shadow(egui::epaint::Shadow {
            offset: egui::vec2(0.0, 2.0),
            blur: 16.0,
            spread: 0.0,
            color: Color32::from_black_alpha(80),
        })
        .stroke(Stroke::new(1.5, Color32::from_rgba_premultiplied(88, 166, 255, 40)));

    card_frame.show(ui, |ui| {
        ui.set_min_height(120.0);
        ui.set_width(available_width);

        let entries = feed.get_entries_with_opacity(now);

        if entries.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(
                    RichText::new("No attempts yet")
                        .color(NEUTRAL_COLOR)
                        .size(SMALL_FONT)
                );
            });
        } else {
            ui.vertical(|ui| {
                for (entry, opacity) in entries.iter().take(5) {
                    let color = match entry.quality {
                        Quality::Perfect => GOOD_COLOR,
                        Quality::Good => WARNING_COLOR,
                        Quality::Failed => BAD_COLOR,
                    };

                    // Apply opacity
                    let faded_color = Color32::from_rgba_premultiplied(
                        color.r(),
                        color.g(),
                        color.b(),
                        (255.0 * opacity) as u8,
                    );

                    let text = format!("{} {}", entry.symbol, entry.message);
                    ui.label(
                        RichText::new(text)
                            .color(faded_color)
                            .size(NORMAL_FONT)
                    );
                }
            });
        }
    });
}

fn render_stats_bar(ui: &mut egui::Ui, stats: &Stats) {
    let available_width = ui.available_width();

    let stats_frame = Frame::none()
        .fill(Color32::from_rgba_premultiplied(18, 22, 32, 180))
        .rounding(Rounding::same(10.0))
        .inner_margin(egui::Margin::symmetric(15.0, 10.0))
        .stroke(Stroke::new(1.0, Color32::from_rgba_premultiplied(88, 166, 255, 30)));

    stats_frame.show(ui, |ui| {
        ui.set_width(available_width);
        ui.horizontal(|ui| {
            ui.add_space(5.0);
            // Perfect count
            ui.label(
                RichText::new(format!("★ {}", stats.perfect_count))
                    .color(GOOD_COLOR)
                    .size(NORMAL_FONT)
                    .strong()
            );

            ui.label(
                RichText::new(format!("{:.0}%", stats.perfect_percentage()))
                    .color(NEUTRAL_COLOR)
                    .size(SMALL_FONT)
            );

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            // Good count
            ui.label(
                RichText::new(format!("● {}", stats.good_count))
                    .color(WARNING_COLOR)
                    .size(NORMAL_FONT)
                    .strong()
            );

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            // Failed count
            ui.label(
                RichText::new(format!("✕ {}", stats.failed_count))
                    .color(BAD_COLOR)
                    .size(NORMAL_FONT)
                    .strong()
            );
        });
    });
}

fn render_controls_hint(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add_space(5.0);

        ui.label(
            RichText::new("⌨")
                .color(ACCENT_COLOR)
                .size(NORMAL_FONT)
        );

        ui.add_space(5.0);

        ui.label(
            RichText::new("A/D")
                .color(TEXT_COLOR)
                .size(SMALL_FONT)
                .strong()
        );

        ui.label(
            RichText::new("practice")
                .color(NEUTRAL_COLOR)
                .size(SMALL_FONT)
        );

        ui.add_space(10.0);
        ui.label(RichText::new("•").color(NEUTRAL_COLOR));
        ui.add_space(10.0);

        ui.label(
            RichText::new("ESC")
                .color(TEXT_COLOR)
                .size(SMALL_FONT)
                .strong()
        );

        ui.label(
            RichText::new("quit")
                .color(NEUTRAL_COLOR)
                .size(SMALL_FONT)
        );
    });
}
