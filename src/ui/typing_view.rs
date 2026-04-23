use crate::state::{AppState, CharResult, CursorStyle, MenuState, TypingState};
use crate::theme::*;
use crate::words::generate_snippet;
use egui::{
    text::LayoutJob, Color32, Direction, Event, FontId, Key, Layout, RichText, TextFormat, Ui, Vec2,
};

const TYPING_AREA_ID: &str = "typing_focus";

pub fn show(ui: &mut Ui, state: &mut TypingState, accent: Color32, cursor_style: CursorStyle) -> Option<AppState> {
    let focus_id = egui::Id::new(TYPING_AREA_ID);
    ui.memory_mut(|m| m.request_focus(focus_id));

    let mut transition = None;

    ui.input(|i| {
        for ev in &i.events {
            match ev {
                Event::Text(s) => {
                    for ch in s.chars() {
                        state.handle_char(ch);
                    }
                }
                Event::Key {
                    key: Key::Backspace,
                    pressed: true,
                    ..
                } => {
                    state.handle_backspace();
                }
                Event::Key {
                    key: Key::Escape,
                    pressed: true,
                    ..
                } => {
                    transition = Some(AppState::Menu(MenuState::default()));
                }
                Event::Key {
                    key: Key::Tab,
                    pressed: true,
                    ..
                } => {
                    let word_count = state.snippet_length.word_count();
                    let snippet = generate_snippet(state.language.name(), word_count);
                    transition = Some(AppState::Typing(TypingState::new(
                        state.language.clone(),
                        state.snippet_length,
                        snippet,
                    )));
                }
                _ => {}
            }
        }
    });

    if state.is_complete() && transition.is_none() {
        transition = Some(AppState::Results(state.into_results_cloned()));
    }

    if state.started_at.is_some() && state.finished_at.is_none() {
        ui.ctx().request_repaint();
    }

    let avail = ui.available_size();

    ui.allocate_ui_with_layout(
        avail,
        Layout::centered_and_justified(Direction::TopDown),
        |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(avail.y * 0.10);
                ui.horizontal(|ui| {
                    let bar_width = (avail.x * 0.55).min(700.0);
                    ui.add_space((avail.x - bar_width) / 2.0);

                    let elapsed = state.elapsed_secs();
                    let wpm = state.wpm();
                    let acc = state.accuracy();
                    let lang = state.language.label().to_string();

                    ui.label(
                        RichText::new(format!("{:.0} wpm", wpm))
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(accent),
                    );
                    ui.add_space(24.0);
                    ui.label(
                        RichText::new(format!("{:.0}%", acc))
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(COLOR_CORRECT),
                    );
                    ui.add_space(24.0);
                    ui.label(
                        RichText::new(format!("{:.1}s", elapsed))
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(COLOR_SUBTEXT),
                    );
                    ui.add_space(24.0);
                    ui.label(
                        RichText::new(lang)
                            .font(FontId::monospace(FONT_SIZE_STATS))
                            .color(COLOR_MUTED),
                    );
                });

                ui.add_space(40.0);

                let snippet_width = (avail.x * 0.70).min(860.0);
                let font = FontId::monospace(FONT_SIZE_SNIPPET);
                let mut job = LayoutJob::default();
                job.wrap = egui::text::TextWrapping {
                    max_width: snippet_width,
                    ..Default::default()
                };

                for (i, &ch) in state.snippet.iter().enumerate() {
                    let color = if i < state.cursor {
                        match state.input[i] {
                            CharResult::Correct => COLOR_CORRECT,
                            CharResult::Incorrect => COLOR_INCORRECT,
                        }
                    } else {
                        COLOR_UNTYPED
                    };

                    let mut fmt = TextFormat {
                        font_id: font.clone(),
                        color,
                        ..Default::default()
                    };

                    if i < state.cursor {
                        if let CharResult::Incorrect = state.input[i] {
                            fmt.background = Color32::from_rgba_unmultiplied(202, 71, 84, 40);
                        }
                    }

                    job.append(&ch.to_string(), 0.0, fmt);
                }

                ui.allocate_ui_with_layout(
                    Vec2::new(snippet_width, avail.y * 0.5),
                    Layout::left_to_right(egui::Align::TOP),
                    |ui| {
                        let _ = ui
                            .allocate_response(Vec2::ZERO, egui::Sense::focusable_noninteractive());
                        let galley = ui.fonts(|f| f.layout_job(job));
                        let (rect, _) = ui.allocate_exact_size(galley.size(), egui::Sense::hover());
                        ui.painter().galley(rect.min, galley.clone(), COLOR_UNTYPED);

                        // get cursor target rect
                        let pcursor = egui::epaint::text::cursor::PCursor { paragraph: 0, offset: state.cursor, prefer_next_row: false };
                        let cursor_rect_local = galley.pos_from_pcursor(pcursor);
                        let target_cursor_rect = cursor_rect_local.translate(rect.min.to_vec2());

                        let target_pos = target_cursor_rect.min;
                        let target_width = if target_cursor_rect.width() > 0.1 { target_cursor_rect.width() } else { 10.0 };

                        if Some(target_pos) != state.last_cursor_target {
                            state.cursor_anim_start_pos = state.cursor_anim_pos.or(Some(target_pos));
                            state.cursor_anim_start_width = state.cursor_anim_width.or(Some(target_width));
                            state.cursor_anim_start_time = Some(std::time::Instant::now());
                            state.last_cursor_target = Some(target_pos);
                        }

                        // NOTE: manually tune animation speed, change `anim_duration` below
                        // Lower value = faster animation. E.g., 0.05 is 50ms.
                        let anim_duration = 0.10;
                        let mut current_pos = target_pos;
                        let mut current_width = target_width;

                        if let (Some(start_pos), Some(start_width), Some(start_time)) = (
                            state.cursor_anim_start_pos,
                            state.cursor_anim_start_width,
                            state.cursor_anim_start_time
                        ) {
                            let elapsed = start_time.elapsed().as_secs_f32();
                            let t = (elapsed / anim_duration).clamp(0.0, 1.0);
                            
                            // easeOutExpo function
                            let eased_t = if t >= 1.0 { 1.0 } else { 1.0 - (-10.0 * t).exp2() };

                            current_pos = start_pos.lerp(target_pos, eased_t);
                            current_width = start_width + (target_width - start_width) * eased_t;

                            if t < 1.0 {
                                ui.ctx().request_repaint();
                            }
                        }

                        state.cursor_anim_pos = Some(current_pos);
                        state.cursor_anim_width = Some(current_width);

                        let current_cursor_rect = egui::Rect::from_min_size(
                            current_pos,
                            egui::vec2(current_width, target_cursor_rect.height()),
                        );

                        match cursor_style {
                            CursorStyle::Bar => {
                                ui.painter().line_segment(
                                    [current_cursor_rect.left_top(), current_cursor_rect.left_bottom()],
                                    egui::Stroke::new(2.0, accent),
                                );
                            }
                            CursorStyle::Underline => {
                                ui.painter().line_segment(
                                    [current_cursor_rect.left_bottom(), current_cursor_rect.right_bottom()],
                                    egui::Stroke::new(2.0, accent),
                                );
                            }
                            CursorStyle::Block => {
                                ui.painter().rect_filled(
                                    current_cursor_rect,
                                    0.0,
                                    accent.gamma_multiply(0.4),
                                );
                            }
                        }
                    },
                );

                ui.add_space(24.0);
                ui.label(
                    RichText::new("esc — menu • tab — restart")
                        .font(FontId::monospace(12.0))
                        .color(COLOR_MUTED),
                );
            });
        },
    );

    transition
}
