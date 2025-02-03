use std::sync::Arc;

use eframe::CreationContext;
use egui::{Color32, Frame, RichText, TopBottomPanel};

use crate::{
    view::{setting_view::Settings, BitcoinView},
    StockTrackerView,
};

pub struct WealthTracker {
    stock_tracker: StockTrackerView,
    btc_tracker: BitcoinView,
    settings: Settings,
}

impl WealthTracker {
    pub fn name() -> &'static str {
        "Wealth Tracker"
    }

    pub fn new(cc: &CreationContext) -> Self {
        load_font(&cc.egui_ctx);

        Self {
            stock_tracker: StockTrackerView::new(cc),
            btc_tracker: BitcoinView::new(),
            settings: Settings::default(),
        }
    }

    fn setting_pannel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let is_open = self.settings.open || ctx.memory(|mem| mem.everything_is_visible());
        egui::SidePanel::left("setting_pannel")
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("💻 Settings");
                });

                ui.separator();
                self.settings.show(ui, frame);
            });
    }

    fn render_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel")
            // .frame(
            //     egui::Frame::default()
            //         .fill(ui.style().visuals.window_fill) // 使用主题背景色
            //         .inner_margin(4.0),
            // )
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("💰")
                            .text_style(egui::TextStyle::Body)
                            .color(Color32::GOLD),
                    );
                    egui::widgets::global_theme_preference_switch(ui);
                    ui.toggle_value(&mut self.settings.open, "💻 Setting");
                });
            });
    }
}

impl eframe::App for WealthTracker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.render_top_panel(ctx);
        self.setting_pannel(ctx, frame);
        egui::CentralPanel::default()
            // .frame(Frame::none())
            .show(ctx, |_ui| {
                // let mut is_open = true;
                self.stock_tracker.show(ctx, &mut self.settings.zh_share);
                self.btc_tracker.show(ctx, &mut self.settings.btc);
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
