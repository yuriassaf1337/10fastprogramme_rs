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
    let avail = ui.available_size();
    
    // main content always takes full available width
    let transition = show_main_content(ui, state, accent, avail);
    
    egui::Area::new(egui::Id::new("hamburger_area"))
        .fixed_pos(egui::Pos2::new(10.0, 10.0))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            let (rect, resp) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::click());
            let color = if resp.hovered() || state.sidebar_open {
                accent
            } else {
                COLOR_MUTED
            };
            draw_hamburger(ui.painter(), rect.center(), color);
            if resp.clicked() {
                state.sidebar_open = !state.sidebar_open;
            }
        });

    if state.sidebar_open {
        let sidebar_width = (avail.x * 0.30).min(300.0).max(200.0);
        let popup_pos = egui::Pos2::new(10.0, 48.0);

        let window_res = egui::Window::new("__sidebar__")
            .title_bar(false)
            .resizable(false)
            .fixed_pos(popup_pos)
            .frame(Frame {
                fill: COLOR_SURFACE,
                rounding: Rounding::same(8.0),
                stroke: Stroke::new(1.0, COLOR_MUTED),
                inner_margin: Margin::same(14.0),
                ..Default::default()
            })
            .show(ui.ctx(), |ui| {
                ui.set_width(sidebar_width);
                ui.set_max_height(avail.y - 60.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    show_sidebar_content(ui, records, accent);
                });
            });

        let clicked_outside = ui.input(|i| {
            if i.pointer.any_pressed() {
                if let Some(pos) = i.pointer.interact_pos() {
                    if let Some(res) = &window_res {
                        let hamburger_rect = egui::Rect::from_min_size(egui::Pos2::new(10.0, 10.0), Vec2::splat(28.0));
                        return !res.response.rect.contains(pos) && !hamburger_rect.contains(pos);
                    }
                }
            }
            false
        });

        if clicked_outside {
            state.sidebar_open = false;
        }
    }

    transition
}

fn draw_hamburger(painter: &egui::Painter, center: egui::Pos2, color: Color32) {
    let w = 18.0;
    let h = 2.0;
    let spacing = 6.0;
    
    for i in -1..=1 {
        let y = center.y + (i as f32) * spacing;
        painter.line_segment(
            [egui::pos2(center.x - w / 2.0, y), egui::pos2(center.x + w / 2.0, y)],
            Stroke::new(h, color),
        );
    }
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
