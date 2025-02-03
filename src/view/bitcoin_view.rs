use std::collections::HashMap;

use egui::{Color32, Frame, Grid, Id, Label, RichText};
use egui_dnd::{dnd, Handle};

#[derive(Hash)]
pub struct Bitcoin {
    name: String,
    amount: String,
}

#[derive(Default)]
pub struct BitcoinView {
    items: Vec<String>,
    data: HashMap<String, Bitcoin>,
}

/// What is being dragged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    row: usize,
}

impl BitcoinView {
    fn name(&self) -> &'static str {
        "BTC"
    }

    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert(
            String::from("BTC"),
            Bitcoin {
                name: "BTC".to_string(),
                amount: "88.88".to_string(),
            },
        );
        m.insert(
            String::from("ETH"),
            Bitcoin {
                name: "ETH".to_string(),
                amount: "48.88".to_string(),
            },
        );
        Self {
            items: vec![
                "alfred".to_string(),
                "bernhard".to_string(),
                "christian".to_string(),
            ],
            data: m,
        }
    }
}

impl BitcoinView {
    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            // .frame(frame)
            .open(open)
            .resizable([true, true])
            .scroll(true)
            .show(ctx, |ui| {
                self.render_coins(ui);
            });
    }

    pub fn render_coins(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);

        let mut items = self.data.values_mut();
        let response = dnd(ui, "stock list").show(&mut items, |ui, item, handle, state| {
            BitcoinView::render_item(item, ui, handle);
        });
        if response.is_drag_finished() {
            response.update_vec(&mut self.items);
        }

        // Grid::new("Bitcoin list")
        //     .max_col_width(60.0)
        //     .striped(true)
        //     .show(ui, |ui| {

        // self.data.values_mut().enumerate().for_each(|(idx, stock)| {
        //     let frame = Frame::default().inner_margin(4.0);

        //     ui.dnd_drop_zone::<Location, ()>(frame, |ui| {
        //         // row
        //         let item_id = Id::new(("drag_and_drop", idx));

        //         let item_location = Location { row: idx };

        //         let response = ui
        //             .dnd_drag_source(item_id, item_location, |ui| {
        //                 // name
        //                 ui.centered_and_justified(|ui| {
        //                     ui.add(
        //                         Label::new(
        //                             RichText::new(stock.name.to_string())
        //                                 .text_style(egui::TextStyle::Body),
        //                         )
        //                         .wrap_mode(egui::TextWrapMode::Truncate),
        //                     );
        //                 });

        //                 // amount
        //                 ui.centered_and_justified(|ui| {
        //                     ui.add(Label::new(
        //                         RichText::new(stock.data_new().to_string())
        //                             .text_style(egui::TextStyle::Body),
        //                     ));
        //                 });

        //                 let color = match stock.data_rise_per() {
        //                     p if p < 0.0 => Color32::GREEN,
        //                     n if n > 0.0 => Color32::RED,
        //                     _ => Color32::WHITE,
        //                 };

        //                 ui.centered_and_justified(|ui| {
        //                     ui.add(Label::new(
        //                         RichText::new(stock.data_rise_per().to_string())
        //                             .text_style(egui::TextStyle::Body)
        //                             .color(color),
        //                     ));
        //                 });
        //             })
        //             .response;
        //     });
        // });
        // });
    }

    pub fn render_item(coin: &mut Bitcoin, ui: &mut egui::Ui, handle: Handle) {
        ui.horizontal(|ui| {
            handle.ui(ui, |ui| {
                // name
                ui.add(
                    Label::new(
                        RichText::new(coin.name.to_string()).text_style(egui::TextStyle::Body),
                    )
                    .wrap_mode(egui::TextWrapMode::Truncate),
                );
            });

            // amount
            ui.add(Label::new(
                RichText::new(coin.amount.to_string()).text_style(egui::TextStyle::Body),
            ));
        });
    }
}
