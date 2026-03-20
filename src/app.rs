use crate::state::{AppState, MenuState};
use crate::theme;
use crate::ui;

pub struct App {
    state: AppState,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_theme(&cc.egui_ctx);
        theme::load_fonts(&cc.egui_ctx);

        Self {
            state: AppState::Menu(MenuState::default()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let transition = match &mut self.state {
                AppState::Menu(s)    => ui::menu::show(ui, s),
                AppState::Typing(s)  => ui::typing_view::show(ui, s),
                AppState::Results(s) => ui::results::show(ui, s),
            };

            if let Some(new_state) = transition {
                self.state = new_state;
            }
        });
    }
}
