use crate::state::{AppState, MenuState};
use crate::theme;
use crate::ui;
use egui::{
    Area, Color32, FontId, Frame, Id, Margin, Order, Pos2, RichText, Rounding, Sense, Stroke, Vec2,
};

pub struct App {
    state: AppState,
    accent: Color32,
    settings_open: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_theme(&cc.egui_ctx);
        theme::load_fonts(&cc.egui_ctx);

        let accent = cc
            .storage
            .and_then(|s| s.get_string("accent"))
            .and_then(|h| parse_hex(&h))
            .unwrap_or(theme::COLOR_ACCENT);

        Self {
            state: AppState::Menu(MenuState::default()),
            accent,
            settings_open: false,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let hex = format!(
            "{:02X}{:02X}{:02X}",
            self.accent.r(),
            self.accent.g(),
            self.accent.b()
        );
        storage.set_string("accent", hex);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let transition = match &mut self.state {
                AppState::Menu(s) => ui::menu::show(ui, s, self.accent),
                AppState::Typing(s) => ui::typing_view::show(ui, s, self.accent),
                AppState::Results(s) => ui::results::show(ui, s, self.accent),
            };

            if let Some(new_state) = transition {
                self.state = new_state;
            }
        });

        let screen = ctx.screen_rect();

        Area::new(Id::new("gear_area"))
            .fixed_pos(Pos2::new(screen.right() - 44.0, 10.0))
            .order(Order::Foreground)
            .show(ctx, |ui| {
                let (rect, resp) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::click());
                let color = if resp.hovered() || self.settings_open {
                    self.accent
                } else {
                    theme::COLOR_MUTED
                };
                draw_gear(ui.painter(), rect.center(), color);
                if resp.clicked() {
                    self.settings_open = !self.settings_open;
                }
            });

        // settings popup
        if self.settings_open {
            let popup_w = 230.0;
            let popup_pos = Pos2::new(screen.right() - popup_w - 10.0, 48.0);

            egui::Window::new("__settings__")
                .title_bar(false)
                .resizable(false)
                .fixed_pos(popup_pos)
                .fixed_size(Vec2::new(popup_w, 0.0))
                .frame(Frame {
                    fill: theme::COLOR_SURFACE,
                    rounding: Rounding::same(8.0),
                    stroke: Stroke::new(1.0, theme::COLOR_MUTED),
                    inner_margin: Margin::same(14.0),
                    ..Default::default()
                })
                .show(ctx, |ui| {
                    ui.label(
                        RichText::new("settings")
                            .font(FontId::monospace(13.0))
                            .color(theme::COLOR_SUBTEXT),
                    );
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("accent color")
                                .font(FontId::monospace(12.0))
                                .color(theme::COLOR_SUBTEXT),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!(
                                    "#{:02X}{:02X}{:02X}",
                                    self.accent.r(),
                                    self.accent.g(),
                                    self.accent.b()
                                ))
                                .font(FontId::monospace(11.0))
                                .color(theme::COLOR_MUTED),
                            );
                        });
                    });
                    ui.add_space(6.0);
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut self.accent,
                        egui::color_picker::Alpha::Opaque,
                    );
                });
        }
    }
}

fn draw_gear(painter: &egui::Painter, center: Pos2, color: Color32) {
    use std::f32::consts::PI;
    let body_r = 6.5_f32;
    let tooth_r = 3.0_f32;
    let hole_r = body_r * 0.40;
    let n = 6usize;

    painter.circle_filled(center, body_r, color);
    for i in 0..n {
        let angle = 2.0 * PI * i as f32 / n as f32;
        let tc = center + egui::Vec2::new(angle.cos(), angle.sin()) * (body_r + tooth_r * 0.65);
        painter.circle_filled(tc, tooth_r, color);
    }
    painter.circle_filled(center, hole_r, theme::COLOR_BG);
}

fn parse_hex(s: &str) -> Option<Color32> {
    let s = s.trim_start_matches('#');
    if s.len() == 6 {
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Color32::from_rgb(r, g, b))
    } else {
        None
    }
}
