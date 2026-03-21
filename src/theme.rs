use egui::{Color32, FontDefinitions};

pub const COLOR_BG: Color32 = Color32::from_rgb(18, 18, 20); // near-black
pub const COLOR_SURFACE: Color32 = Color32::from_rgb(30, 30, 34); // slightly lighter

pub const COLOR_UNTYPED: Color32 = Color32::from_rgb(100, 100, 110); // dim gray
pub const COLOR_CORRECT: Color32 = Color32::from_rgb(230, 230, 230); // near-white
pub const COLOR_INCORRECT: Color32 = Color32::from_rgb(202, 71, 84); // soft red

// UI accents
pub const COLOR_ACCENT: Color32 = Color32::from_rgb(230, 182, 58); // yellow
pub const COLOR_MUTED: Color32 = Color32::from_rgb(70, 70, 80);
pub const COLOR_SUBTEXT: Color32 = Color32::from_rgb(130, 130, 145);

pub const FONT_SIZE_SNIPPET: f32 = 24.0;
pub const FONT_SIZE_STATS: f32 = 18.0;
pub const FONT_SIZE_LABEL: f32 = 14.0;

pub fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.window_fill = COLOR_BG;
    visuals.panel_fill = COLOR_BG;
    visuals.faint_bg_color = COLOR_SURFACE;
    visuals.extreme_bg_color = COLOR_BG;
    visuals.override_text_color = Some(COLOR_UNTYPED);

    visuals.widgets.noninteractive.bg_fill = COLOR_SURFACE;
    visuals.widgets.inactive.bg_fill = COLOR_SURFACE;
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(45, 45, 52);
    visuals.widgets.active.bg_fill = Color32::from_rgb(55, 55, 64);

    visuals.selection.bg_fill = COLOR_ACCENT.gamma_multiply(0.25);
    visuals.selection.stroke = egui::Stroke::new(1.0, COLOR_ACCENT);

    visuals.window_rounding = egui::Rounding::same(8.0);
    visuals.window_shadow = egui::epaint::Shadow::NONE;

    ctx.set_visuals(visuals);
}

// todo: implement custom fonts
pub fn load_fonts(ctx: &egui::Context) {
    ctx.set_fonts(FontDefinitions::default());
}
