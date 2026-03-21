use crate::state::{AppState, CharResult, MenuState, TypingState};
use crate::theme::*;
use crate::words::generate_snippet;
use egui::{
    text::LayoutJob, Color32, Direction, Event, FontId, Key, Layout, RichText, TextFormat, Ui, Vec2,
};

const TYPING_AREA_ID: &str = "typing_focus";

pub fn show(ui: &mut Ui, state: &mut TypingState, accent: Color32) -> Option<AppState> {
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

                    if i == state.cursor {
                        fmt.underline = egui::Stroke::new(2.0, accent);
                    }

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
                        ui.painter().galley(rect.min, galley, COLOR_UNTYPED);
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
