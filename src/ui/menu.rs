use crate::state::{AppState, Language, MenuState, SnippetLength, TypingState};
use crate::theme::*;
use crate::words::generate_snippet;
use egui::{Color32, Direction, FontId, Layout, RichText, Ui, Vec2};

pub fn show(ui: &mut Ui, state: &mut MenuState, accent: Color32) -> Option<AppState> {
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
                        .color(accent),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new("a typing trainer 4 programmers")
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
                    let btn_width = 90.0;
                    let total_width = state.languages.len() as f32 * btn_width;
                    ui.add_space((ui.available_width() - total_width) / 2.0);

                    let languages: Vec<Language> = state.languages.clone();
                    for lang in &languages {
                        let selected = state.selected_language == *lang;
                        let color = if selected { accent } else { COLOR_SUBTEXT };
                        let text = RichText::new(lang.label())
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(color);
                        if ui
                            .add(
                                egui::Button::new(text)
                                    .min_size(Vec2::new(80.0, 32.0))
                                    .fill(if selected {
                                        accent.gamma_multiply(0.15)
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .stroke(egui::Stroke::new(
                                        1.0,
                                        if selected { accent } else { COLOR_MUTED },
                                    )),
                            )
                            .clicked()
                        {
                            state.selected_language = lang.clone();
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
                    let lengths = [
                        SnippetLength::Short,
                        SnippetLength::Medium,
                        SnippetLength::Long,
                    ];
                    ui.add_space(ui.available_width() / 2.0 - (lengths.len() as f32 * 70.0) / 2.0);
                    for len in lengths {
                        let selected = state.snippet_length == len;
                        let color = if selected { accent } else { COLOR_SUBTEXT };
                        let text = RichText::new(len.label())
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(color);
                        if ui
                            .add(
                                egui::Button::new(text)
                                    .min_size(Vec2::new(60.0, 32.0))
                                    .fill(if selected {
                                        accent.gamma_multiply(0.15)
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .stroke(egui::Stroke::new(
                                        1.0,
                                        if selected { accent } else { COLOR_MUTED },
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
                            .fill(accent)
                            .stroke(egui::Stroke::NONE),
                    )
                    .clicked()
                {
                    let word_count = state.snippet_length.word_count();
                    let snippet = generate_snippet(state.selected_language.name(), word_count);
                    transition = Some(AppState::Typing(TypingState::new(
                        state.selected_language.clone(),
                        state.snippet_length,
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
