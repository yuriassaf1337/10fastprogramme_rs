use crate::state::{AppState, MenuState, ResultsData, TypingState};
use crate::theme::*;
use crate::words::generate_snippet;
use egui::epaint::{PathShape, PathStroke};
use egui::{
    Align2, Color32, Direction, FontId, Layout, Pos2, Rect, RichText, Sense, Shape, Stroke, Ui,
    Vec2,
};

const COLOR_ACC_LINE: Color32 = Color32::from_rgb(210, 210, 220);

pub fn show(ui: &mut Ui, data: &ResultsData, accent: Color32) -> Option<AppState> {
    let mut transition = None;
    let avail = ui.available_size();

    ui.allocate_ui_with_layout(
        avail,
        Layout::centered_and_justified(Direction::TopDown),
        |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(avail.y * 0.06);

                ui.label(
                    RichText::new("results")
                        .font(FontId::monospace(14.0))
                        .color(COLOR_SUBTEXT),
                );
                ui.add_space(8.0);

                ui.label(
                    RichText::new(format!("{:.0}", data.wpm))
                        .font(FontId::monospace(72.0))
                        .color(accent),
                );
                ui.label(
                    RichText::new("wpm")
                        .font(FontId::monospace(FONT_SIZE_LABEL))
                        .color(COLOR_SUBTEXT),
                );

                ui.add_space(20.0);

                // stats row
                ui.horizontal(|ui| {
                    let block_w = 100.0;
                    let gap = 24.0;
                    let num = 5u32;
                    let total = block_w * num as f32 + gap * (num - 1) as f32;
                    ui.add_space(((avail.x - total) / 2.0).max(0.0));

                    for (val, lbl) in &[
                        (format!("{:.1}%", data.accuracy), "accuracy"),
                        (format!("{:.1}s", data.time_elapsed), "time"),
                        (data.errors.to_string(), "errors"),
                        (data.language.label().to_string(), "language"),
                        (data.snippet_length.label().to_string(), "words"),
                    ] {
                        ui.allocate_ui(Vec2::new(block_w, 56.0), |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new(val)
                                        .font(FontId::monospace(22.0))
                                        .color(COLOR_CORRECT),
                                );
                                ui.label(
                                    RichText::new(*lbl)
                                        .font(FontId::monospace(FONT_SIZE_LABEL))
                                        .color(COLOR_SUBTEXT),
                                );
                            });
                        });
                        ui.add_space(gap);
                    }
                });

                ui.add_space(16.0);

                if data.wpm_history.len() >= 2 {
                    draw_wpm_graph(
                        ui,
                        &data.wpm_history,
                        &data.accuracy_history,
                        avail.x,
                        accent,
                    );
                }

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    let legend_w = 180.0;
                    ui.add_space(((avail.x - legend_w) / 2.0).max(0.0));
                    legend_dot(ui, accent, "wpm");
                    ui.add_space(24.0);
                    legend_dot(ui, COLOR_ACC_LINE, "accuracy");
                });

                ui.add_space(20.0);

                // buttons
                ui.horizontal(|ui| {
                    ui.add_space((avail.x - 332.0) / 2.0);

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("retry")
                                    .font(FontId::monospace(18.0))
                                    .color(COLOR_BG),
                            )
                            .min_size(Vec2::new(150.0, 42.0))
                            .fill(accent)
                            .stroke(Stroke::NONE),
                        )
                        .clicked()
                    {
                        let word_count = data.snippet_length.word_count();
                        let snippet = generate_snippet(data.language.name(), word_count);
                        transition = Some(AppState::Typing(TypingState::new(
                            data.language.clone(),
                            data.snippet_length,
                            snippet,
                        )));
                    }

                    ui.add_space(16.0);

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("menu")
                                    .font(FontId::monospace(18.0))
                                    .color(COLOR_SUBTEXT),
                            )
                            .min_size(Vec2::new(150.0, 42.0))
                            .fill(Color32::TRANSPARENT)
                            .stroke(Stroke::new(1.0, COLOR_MUTED)),
                        )
                        .clicked()
                    {
                        transition = Some(AppState::Menu(MenuState::default()));
                    }
                });
            });
        },
    );

    transition
}

fn draw_wpm_graph(ui: &mut Ui, wpm: &[f32], acc: &[f32], avail_width: f32, accent: Color32) {
    let pad_left = 44.0;
    let pad_right = 44.0;
    let pad_top = 8.0;
    let pad_bottom = 26.0;
    let graph_w = (avail_width * 0.65).clamp(340.0, 700.0);
    let graph_h = 140.0;
    let total_w = graph_w + pad_left + pad_right;
    let total_h = graph_h + pad_top + pad_bottom;

    ui.horizontal(|ui| {
        ui.add_space(((avail_width - total_w) / 2.0).max(0.0));
        let (resp, painter) = ui.allocate_painter(Vec2::new(total_w, total_h), Sense::hover());
        let r = resp.rect;
        let plot = Rect::from_min_max(
            Pos2::new(r.left() + pad_left, r.top() + pad_top),
            Pos2::new(r.right() - pad_right, r.bottom() - pad_bottom),
        );

        let n = wpm.len();
        let max_wpm = wpm.iter().cloned().fold(0.0f32, f32::max);
        let y_max_wpm = ((max_wpm / 50.0).ceil() * 50.0).max(50.0);

        let to_x = |i: usize| plot.left() + (i as f32 / (n - 1) as f32) * plot.width();
        let wpm_y = |w: f32| plot.bottom() - (w / y_max_wpm) * plot.height();
        let acc_y = |a: f32| plot.bottom() - (a / 100.0) * plot.height();

        // grid lines + left y-axis labels in accent color
        let grid_steps = (y_max_wpm / 50.0) as usize;
        for step in 0..=grid_steps {
            let wpm_val = step as f32 * 50.0;
            let y = wpm_y(wpm_val);
            painter.line_segment(
                [Pos2::new(plot.left(), y), Pos2::new(plot.right(), y)],
                Stroke::new(1.0, COLOR_MUTED.gamma_multiply(0.35)),
            );
            painter.text(
                Pos2::new(r.left() + pad_left - 6.0, y),
                Align2::RIGHT_CENTER,
                format!("{}", wpm_val as u32),
                FontId::monospace(10.0),
                accent.gamma_multiply(0.85),
            );
        }

        // right y-axis labels (accuracy %)
        for &pct in &[0u32, 50, 100] {
            let y = acc_y(pct as f32);
            painter.text(
                Pos2::new(r.right() - pad_right + 6.0, y),
                Align2::LEFT_CENTER,
                format!("{}%", pct),
                FontId::monospace(10.0),
                COLOR_ACC_LINE.gamma_multiply(0.6),
            );
        }

        // x-axis labels
        let label_every = ((n as f32 / 7.0).ceil() as usize).max(1);
        for i in (0..n).step_by(label_every) {
            painter.text(
                Pos2::new(to_x(i), plot.bottom() + 5.0),
                Align2::CENTER_TOP,
                format!("{}s", i + 1),
                FontId::monospace(10.0),
                COLOR_SUBTEXT,
            );
        }

        // WPM area fill
        let wpm_pts: Vec<Pos2> = (0..n).map(|i| Pos2::new(to_x(i), wpm_y(wpm[i]))).collect();
        let mut fill = wpm_pts.clone();
        fill.push(Pos2::new(plot.right(), plot.bottom()));
        fill.push(Pos2::new(plot.left(), plot.bottom()));
        painter.add(Shape::Path(PathShape {
            points: fill,
            closed: true,
            fill: accent.gamma_multiply(0.13),
            stroke: PathStroke::NONE,
        }));

        // WPM line + dots
        painter.add(Shape::line(wpm_pts.clone(), Stroke::new(2.0, accent)));
        for pt in &wpm_pts {
            painter.circle_filled(*pt, 6.0, accent.gamma_multiply(0.2));
            painter.circle_filled(*pt, 3.5, accent);
        }

        // accuracy line + dots
        if acc.len() == n {
            let acc_pts: Vec<Pos2> = (0..n).map(|i| Pos2::new(to_x(i), acc_y(acc[i]))).collect();
            painter.add(Shape::line(
                acc_pts.clone(),
                Stroke::new(2.0, COLOR_ACC_LINE),
            ));
            for pt in &acc_pts {
                painter.circle_filled(*pt, 6.0, COLOR_ACC_LINE.gamma_multiply(0.15));
                painter.circle_filled(*pt, 3.0, COLOR_ACC_LINE);
            }
        }

        painter.rect_stroke(plot, 0.0, Stroke::new(1.0, COLOR_MUTED.gamma_multiply(0.4)));
    });
}

fn legend_dot(ui: &mut Ui, color: Color32, label: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
        ui.painter().circle_filled(rect.center(), 4.0, color);
        ui.label(
            RichText::new(label)
                .font(FontId::monospace(12.0))
                .color(COLOR_SUBTEXT),
        );
    });
}
