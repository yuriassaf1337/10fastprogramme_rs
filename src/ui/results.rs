use egui::{Direction, FontId, Layout, RichText, Ui, Vec2};
use crate::state::{AppState, MenuState, ResultsData, SnippetLength, TypingState};
use crate::theme::*;
use crate::words::generate_snippet;

pub fn show(ui: &mut Ui, data: &ResultsData) -> Option<AppState> {
    let mut transition = None;
    let avail = ui.available_size();

    ui.allocate_ui_with_layout(avail, Layout::centered_and_justified(Direction::TopDown), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(avail.y * 0.15);

            ui.label(
                RichText::new("results")
                    .font(FontId::monospace(14.0))
                    .color(COLOR_SUBTEXT),
            );
            ui.add_space(32.0);

            ui.label(
                RichText::new(format!("{:.0}", data.wpm))
                    .font(FontId::monospace(72.0))
                    .color(COLOR_ACCENT),
            );
            ui.label(
                RichText::new("wpm")
                    .font(FontId::monospace(FONT_SIZE_LABEL))
                    .color(COLOR_SUBTEXT),
            );

            ui.add_space(32.0);

            ui.horizontal(|ui| {
                let row_width = 480.0_f32.min(avail.x * 0.6);
                ui.add_space((avail.x - row_width) / 2.0);

                stat_block(ui, &format!("{:.1}%", data.accuracy), "accuracy");
                ui.add_space(40.0);
                stat_block(ui, &format!("{:.1}s", data.time_elapsed), "time");
                ui.add_space(40.0);
                stat_block(ui, &data.errors.to_string(), "errors");
                ui.add_space(40.0);
                stat_block(ui, data.language.label(), "language");
            });

            ui.add_space(56.0);

            ui.horizontal(|ui| {
                ui.add_space((avail.x - 332.0) / 2.0);

                // retry same language
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("retry")
                                .font(FontId::monospace(18.0))
                                .color(COLOR_BG),
                        )
                        .min_size(Vec2::new(150.0, 42.0))
                        .fill(COLOR_ACCENT)
                        .stroke(egui::Stroke::NONE),
                    )
                    .clicked()
                {
                    let snippet = generate_snippet(data.language, SnippetLength::Medium);
                    transition = Some(AppState::Typing(TypingState::new(data.language, snippet)));
                }

                ui.add_space(16.0);

                // go back to menu
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("menu")
                                .font(FontId::monospace(18.0))
                                .color(COLOR_SUBTEXT),
                        )
                        .min_size(Vec2::new(150.0, 42.0))
                        .fill(egui::Color32::TRANSPARENT)
                        .stroke(egui::Stroke::new(1.0, COLOR_MUTED)),
                    )
                    .clicked()
                {
                    transition = Some(AppState::Menu(MenuState::default()));
                }
            });
        });
    });

    transition
}

fn stat_block(ui: &mut Ui, value: &str, label: &str) {
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(value)
                .font(FontId::monospace(28.0))
                .color(COLOR_CORRECT),
        );
        ui.label(
            RichText::new(label)
                .font(FontId::monospace(FONT_SIZE_LABEL))
                .color(COLOR_SUBTEXT),
        );
    });
}
