use crate::records::Records;
use crate::state::{AppState, Language, MenuState, SnippetLength, TypingState};
use crate::theme::*;
use crate::words::generate_snippet;
use egui::{Color32, Direction, FontId, Frame, Layout, Margin, RichText, Rounding, Stroke, Ui, Vec2};
use std::collections::HashSet;

pub fn show(
    ui: &mut Ui,
    state: &mut MenuState,
    records: &Records,
    accent: Color32,
) -> Option<AppState> {
    let mut transition = None;
    let avail = ui.available_size();

    // two-column layout if enough width
    let show_sidebar = avail.x > 900.0;
    
    if show_sidebar {
        ui.horizontal(|ui| {
            let sidebar_width = avail.x * 0.30;
            ui.allocate_ui(Vec2::new(sidebar_width, avail.y), |ui| {
                ui.add_space(avail.y * 0.15);
                show_sidebar_content(ui, records, accent);
            });
            
            ui.allocate_ui(Vec2::new(avail.x - sidebar_width, avail.y), |ui| {
                transition = show_main_content(ui, state, accent, avail);
            });
        });
    } else {
        transition = show_main_content(ui, state, accent, avail);
    }

    transition
}

fn show_sidebar_content(ui: &mut Ui, records: &Records, accent: Color32) {
    ui.vertical(|ui| {
        
        ui.label(
            RichText::new("recent runs")
                .font(FontId::monospace(FONT_SIZE_LABEL))
                .color(COLOR_SUBTEXT),
        );
        ui.add_space(8.0);
        
        let recent = records.recent(5);
        if recent.is_empty() {
            ui.label(RichText::new("no runs yet").font(FontId::monospace(12.0)).color(COLOR_MUTED));
        } else {
            for run in recent {
                Frame::none()
                    .fill(COLOR_SURFACE)
                    .rounding(Rounding::same(6.0))
                    .inner_margin(Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&run.language).font(FontId::monospace(12.0)).color(accent));
                            ui.label(RichText::new(&run.snippet_length).font(FontId::monospace(12.0)).color(COLOR_SUBTEXT));
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                if run.completed {
                                    ui.label(RichText::new(format!("{:.0} wpm", run.wpm)).font(FontId::monospace(12.0)).color(COLOR_CORRECT));
                                } else {
                                    ui.label(RichText::new("quit").font(FontId::monospace(12.0)).color(COLOR_INCORRECT));
                                }
                            });
                        });
                    });
                ui.add_space(6.0);
            }
        }
        
        ui.add_space(32.0);
        
        ui.label(
            RichText::new("personal bests")
                .font(FontId::monospace(FONT_SIZE_LABEL))
                .color(COLOR_SUBTEXT),
        );
        ui.add_space(8.0);
        
        let mut has_pb = false;
        let mut pb_frames = Vec::new();
        
        let mut all_combinations: Vec<(String, String)> = records.recent(1000).iter()
            .filter(|r| r.completed)
            .map(|r| (r.language.clone(), r.snippet_length.clone()))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
            
        all_combinations.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        
        for (lang, len) in all_combinations {
            if let Some(pb) = records.personal_best(&lang, &len) {
                has_pb = true;
                pb_frames.push(pb.clone());
            }
        }
        
        if !has_pb {
            ui.label(RichText::new("—").font(FontId::monospace(12.0)).color(COLOR_MUTED));
        } else {
            for pb in pb_frames {
                Frame::none()
                    .fill(COLOR_SURFACE)
                    .rounding(Rounding::same(6.0))
                    .inner_margin(Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&pb.language).font(FontId::monospace(12.0)).color(accent));
                            ui.label(RichText::new(&pb.snippet_length).font(FontId::monospace(12.0)).color(COLOR_SUBTEXT));
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("{:.0} wpm ({:.0}%)", pb.wpm, pb.accuracy)).font(FontId::monospace(12.0)).color(COLOR_CORRECT));
                            });
                        });
                    });
                ui.add_space(6.0);
            }
        }
    });
}

fn show_main_content(ui: &mut Ui, state: &mut MenuState, accent: Color32, total_avail: Vec2) -> Option<AppState> {
    let mut transition = None;
    ui.allocate_ui_with_layout(
        ui.available_size(),
        Layout::centered_and_justified(Direction::TopDown),
        |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(total_avail.y * 0.15);

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
                                    .stroke(Stroke::new(
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
                                    .stroke(Stroke::new(
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
                            .stroke(Stroke::NONE),
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
