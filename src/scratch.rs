pub fn test(galley: std::sync::Arc<egui::Galley>) {
    let p = galley.pos_from_pcursor(egui::epaint::text::cursor::PCursor { paragraph: 0, offset: 0, prefer_next_row: false });
}
