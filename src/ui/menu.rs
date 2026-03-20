use egui::{Direction, FontId, Layout, RichText, Ui, Vec2};
use crate::state::{AppState, Language, MenuState, SnippetLength, TypingState};
use crate::theme::*;
use crate::words::generate_snippet;

pub fn show(ui: &mut Ui, state: &mut MenuState) -> Option<AppState> {
    let mut transition = None;

    let avail = ui.available_size();

    ui.allocate_ui_with_layout(
        avail,
        Layout::centered_and_justified(Direction::TopDown),
        |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(avail.y * 0.15);

                ui.label(
                    RichText::new("10fastprogramme.rs")
                        .font(FontId::monospace(32.0))
                        .color(COLOR_ACCENT),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new("a typing trainer for programmers")
                        .font(FontId::monospace(FONT_SIZE_LABEL))
                        .color(COLOR_SUBTEXT),
                );

                ui.add_space(48.0);

                ui.label(
                    RichText::new("language")
                        .font(FontId::monospace(FONT_SIZE_LABEL))
                        .color(COLOR_SUBTEXT),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 2.0
                        - (Language::all().len() as f32 * 90.0) / 2.0);
                    for lang in Language::all() {
                        let selected = state.selected_language == *lang;
                        let color = if selected { COLOR_ACCENT } else { COLOR_SUBTEXT };
                        let text = RichText::new(lang.label())
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(color);
                        if ui
                            .add(
                                egui::Button::new(text)
                                    .min_size(Vec2::new(80.0, 32.0))
                                    .fill(if selected {
                                        COLOR_ACCENT.gamma_multiply(0.15)
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    })
                                    .stroke(egui::Stroke::new(
                                        1.0,
                                        if selected { COLOR_ACCENT } else { COLOR_MUTED },
                                    )),
                            )
                            .clicked()
                        {
                            state.selected_language = *lang;
                        }
                    }
                });

                ui.add_space(32.0);

                ui.label(
                    RichText::new("words")
                        .font(FontId::monospace(FONT_SIZE_LABEL))
                        .color(COLOR_SUBTEXT),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let lengths = [SnippetLength::Short, SnippetLength::Medium, SnippetLength::Long];
                    ui.add_space(ui.available_width() / 2.0
                        - (lengths.len() as f32 * 70.0) / 2.0);
                    for len in lengths {
                        let selected = state.snippet_length == len;
                        let color = if selected { COLOR_ACCENT } else { COLOR_SUBTEXT };
                        let text = RichText::new(len.label())
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(color);
                        if ui
                            .add(
                                egui::Button::new(text)
                                    .min_size(Vec2::new(60.0, 32.0))
                                    .fill(if selected {
                                        COLOR_ACCENT.gamma_multiply(0.15)
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    })
                                    .stroke(egui::Stroke::new(
                                        1.0,
                                        if selected { COLOR_ACCENT } else { COLOR_MUTED },
                                    )),
                            )
                            .clicked()
                        {
                            state.snippet_length = len;
                        }
                    }
                });

                ui.add_space(48.0);

                let start_text = RichText::new("start")
                    .font(FontId::monospace(20.0))
                    .color(COLOR_BG);
                if ui
                    .add(
                        egui::Button::new(start_text)
                            .min_size(Vec2::new(160.0, 44.0))
                            .fill(COLOR_ACCENT)
                            .stroke(egui::Stroke::NONE),
                    )
                    .clicked()
                {
                    let snippet = generate_snippet(state.selected_language, state.snippet_length);
                    transition = Some(AppState::Typing(TypingState::new(
                        state.selected_language,
                        snippet,
                    )));
                }

                ui.add_space(8.0);
                ui.label(
                    RichText::new("press tab to restart  •  esc to go back")
                        .font(FontId::monospace(12.0))
                        .color(COLOR_MUTED),
                );
            });
        },
    );

    transition
}
