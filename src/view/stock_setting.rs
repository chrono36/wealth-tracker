#[derive(Default)]
pub struct StockSetting {
    pub open: bool,
    pub interval: u32,
    pub search_text: String,
}

impl StockSetting {
    pub fn render(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}
