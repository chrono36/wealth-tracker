use std::sync::Arc;

use eframe::CreationContext;
use egui::{Frame, Layout, TopBottomPanel};

use crate::StockTrackerView;

pub struct WealthTracker {
    stock_tracker: StockTrackerView,
}

impl WealthTracker {
    pub fn name() -> &'static str {
        "Wealth Tracker"
    }

    pub fn new(cc: &CreationContext) -> Self {
        load_font(&cc.egui_ctx);

        Self {
            stock_tracker: StockTrackerView::default(),
        }
    }

    fn render_top_panel(&self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::global_theme_preference_buttons(ui);
        });
    }
}

impl eframe::App for WealthTracker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.render_top_panel(ctx);

        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                let mut is_open = true;
                self.stock_tracker.show(ctx, &mut is_open);
            });
    }
}

fn load_font(ctx: &egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();

    fonts.font_data.insert(
        "AlibabaPuHuiTi-3-55-Regular".to_owned(),
        Arc::new(eframe::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/AlibabaPuHuiTi-3-55-Regular.ttf"
        ))),
    ); // .ttf and .otf supported

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "AlibabaPuHuiTi-3-55-Regular".to_owned());
    ctx.set_fonts(fonts);
}
