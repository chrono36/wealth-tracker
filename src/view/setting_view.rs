use egui::{global_theme_preference_buttons, Color32, RichText};

#[derive(Default)]
pub struct Settings {
    pub open: bool,
    pub zh_share: bool,
    pub hk_share: bool,
    pub us_share: bool,
    pub btc: bool,
}

impl Settings {
    pub fn show(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.label("主题:");
        global_theme_preference_buttons(ui);

        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.label(RichText::new("🚀 Finance:").color(Color32::KHAKI));

                let Self {
                    zh_share,
                    hk_share,
                    us_share,
                    btc,
                    ..
                } = self;

                // ui.toggle_value(
                //     &mut zh_share,
                //     RichText::new("💹 A股").color(Color32::LIGHT_RED),
                // );
                // ui.toggle_value(&mut btc, RichText::new("💰 BTC").color(Color32::ORANGE));

                // ui.toggle_value(
                //     &mut hk_share,
                //     RichText::new("💱 港股").color(Color32::LIGHT_BLUE),
                // );

                // ui.toggle_value(
                //     &mut us_share,
                //     RichText::new("💸 美股").color(Color32::LIGHT_GREEN),
                // );

                ui.checkbox(zh_share, RichText::new("💹 A股").color(Color32::LIGHT_RED));
                ui.checkbox(btc, RichText::new("💰 BTC").color(Color32::ORANGE));
                ui.checkbox(
                    hk_share,
                    RichText::new("💱 港股").color(Color32::LIGHT_BLUE),
                );
                ui.checkbox(
                    us_share,
                    RichText::new("💸 美股").color(Color32::LIGHT_GREEN),
                );
            });
        });
    }
}
