use egui::{
    menu, Align, Align2, Button, Color32, Context, CursorIcon, Grid, Label, Layout, RichText,
    Separator, Shadow, Stroke, TextStyle, TopBottomPanel, Vec2,
};
use egui_plot::{
    AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, HLine, Plot, PlotPoint, Text,
};

use crate::model::{stock, Stock, StockData};

pub struct StockTrackerView {
    stocks: Vec<Stock>,
    show_klines_viewport: bool,
}

impl Default for StockTrackerView {
    fn default() -> Self {
        let stocks = vec![
            Stock {
                name: String::from("比亚迪"),
                code: String::from("sz002594"),
                data: StockData::default(),
                ..Default::default()
            },
            Stock {
                name: String::from("长安汽车"),
                code: String::from("sz000625"),
                data: StockData::default(),
                ..Default::default()
            },
            Stock {
                name: String::from("赛力斯"),
                code: String::from("sh601127"),
                data: StockData::default(),
                ..Default::default()
            },
        ];

        // fetch data

        let codes = vec![
            "sh000001".to_string(),
            "sz399001".to_string(),
            "sh000300".to_string(),
            "sz000625".to_string(),
            "sz002594".to_string(),
            "sh601127".to_string(),
        ];

        let res = stock::fetch_data_list(codes);

        match res {
            Ok(v) => Self {
                stocks: v,
                show_klines_viewport: false,
            },
            Err(e) => Self {
                stocks: stocks,
                show_klines_viewport: false,
            },
        }
    }
}

impl StockTrackerView {
    fn name(&self) -> &'static str {
        "Stock Tracker"
    }

    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        let frame = egui::Frame::none().shadow(Shadow::NONE);
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .frame(frame)
            .open(open)
            .resizable([true, true])
            .scroll(true)
            .show(ctx, |ui| {
                self.refresh();
                self.render_stocks(ctx, ui);
            });
    }
}

impl StockTrackerView {
    fn render_stocks(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.add_space(2.0);

        Grid::new("Stock list")
            .max_col_width(60.0)
            .striped(true)
            .show(ui, |ui| {
                self.stocks.iter_mut().for_each(|stock| {
                    // name
                    ui.centered_and_justified(|ui| {
                        ui.add(
                            Label::new(
                                RichText::new(stock.name.to_string())
                                    .text_style(egui::TextStyle::Body),
                            )
                            .wrap_mode(egui::TextWrapMode::Truncate),
                        );
                    });

                    ui.centered_and_justified(|ui| {
                        ui.add(Label::new(
                            RichText::new(stock.data_new().to_string())
                                .text_style(egui::TextStyle::Body),
                        ));
                    });

                    let color = match stock.data_rise_per() {
                        p if p < 0.0 => Color32::GREEN,
                        n if n > 0.0 => Color32::RED,
                        _ => Color32::WHITE,
                    };

                    ui.centered_and_justified(|ui| {
                        ui.add(Label::new(
                            RichText::new(stock.data_rise_per().to_string())
                                .text_style(egui::TextStyle::Body)
                                .color(color),
                        ));
                    });

                    // let text = match stock.data_rise_per() {
                    //     p if p < 0.0 => RichText::new("📉"),
                    //     n if n > 0.0 => RichText::new("📈"),
                    //     _ => RichText::new(" "),
                    // };
                    // ui.label(text);

                    ui.centered_and_justified(|ui| {
                        let boxs: Vec<BoxElem> = stock
                            .kline
                            .klines
                            .iter()
                            .enumerate()
                            .map(|(i, x)| {
                                let fill_color = if x.close < x.open {
                                    Color32::GREEN
                                } else {
                                    Color32::RED
                                };

                                BoxElem::new(
                                    i as f64,
                                    BoxSpread::new(
                                        x.low,
                                        x.open,
                                        (x.open + x.close) / 2.0,
                                        x.close,
                                        x.high,
                                    ),
                                )
                                .stroke(Stroke::new(0.2, fill_color))
                                .fill(fill_color.linear_multiply(0.1))
                                .box_width(0.8)
                            })
                            .collect();

                        let box1 = BoxPlot::new(boxs);

                        let plot = Plot::new(format!("{}_kline", stock.code))
                            .allow_zoom(false)
                            .allow_drag(false)
                            .allow_scroll(false)
                            .show_grid([false, false])
                            .show_axes([false, false])
                            .sharp_grid_lines(false)
                            // .show_background(false)
                            .width(50.0)
                            .height(16.0)
                            .show(ui, |plot_ui| {
                                plot_ui.box_plot(box1);
                            })
                            .response;

                        if plot.clicked() {
                            // request klines

                            self.show_klines_viewport = true;
                        }

                        if self.show_klines_viewport {
                            ctx.show_viewport_immediate(
                                egui::ViewportId::from_hash_of(format!("{}_kline_v", stock.code)),
                                egui::ViewportBuilder::default()
                                    .with_title(format!("{}", stock.code)),
                                |ctx, _class| {
                                    egui::CentralPanel::default().show(ctx, |ui| {
                                        ui.vertical(|ui| {
                                            ui.horizontal_wrapped(|ui| {
                                                if ui
                                                    .selectable_value(
                                                        &mut stock.kline.scale,
                                                        stock::KLineScale::Munute5,
                                                        "5",
                                                    )
                                                    .clicked()
                                                {
                                                    //TODO: request scale kline data
                                                }
                                                if ui
                                                    .selectable_value(
                                                        &mut stock.kline.scale,
                                                        stock::KLineScale::Munute15,
                                                        "15",
                                                    )
                                                    .clicked()
                                                {
                                                    //TODO: request scale kline data
                                                }
                                                if ui
                                                    .selectable_value(
                                                        &mut stock.kline.scale,
                                                        stock::KLineScale::Munute30,
                                                        "30",
                                                    )
                                                    .clicked()
                                                {
                                                    //TODO: request scale kline data
                                                }
                                                if ui
                                                    .selectable_value(
                                                        &mut stock.kline.scale,
                                                        stock::KLineScale::Day,
                                                        "day",
                                                    )
                                                    .clicked()
                                                {
                                                    //TODO: request scale kline data
                                                }
                                            });

                                            //
                                            let mut x_axes = vec![];
                                            let boxs: Vec<BoxElem> = stock
                                                .kline
                                                .klines
                                                .iter()
                                                .enumerate()
                                                .map(|(i, x)| {
                                                    let x_hints = AxisHints::new_x().label("Time");
                                                    x_axes.push(x_hints);

                                                    let fill_color = if x.close < x.open {
                                                        Color32::GREEN
                                                    } else {
                                                        Color32::RED
                                                    };
                                                    BoxElem::new(
                                                        i as f64,
                                                        BoxSpread::new(
                                                            x.low,
                                                            x.open,
                                                            (x.open + x.close) / 2.0,
                                                            x.close,
                                                            x.high,
                                                        ),
                                                    )
                                                    .name(x.date.clone())
                                                    .stroke(Stroke::new(0.2, fill_color))
                                                    .fill(fill_color.linear_multiply(0.05))
                                                    .box_width(0.8)
                                                })
                                                .collect();
                                            let box1 = BoxPlot::new(boxs);
                                            Plot::new(format!("{}_kline", stock.code))
                                                .show_background(false)
                                                .show_grid(true)
                                                .allow_drag([true, false])
                                                .custom_x_axes(vec![
                                                    AxisHints::new_x().label("Time (s)")
                                                ])
                                                .show(ui, |plot_ui| {
                                                    plot_ui.box_plot(box1);
                                                })
                                                .response;
                                        });
                                    });
                                    if ctx.input(|i| i.viewport().close_requested()) {
                                        // Tell parent viewport that we should not show next frame:
                                        self.show_klines_viewport = false;
                                    }
                                },
                            );
                        }
                    });

                    ui.centered_and_justified(|ui| {
                        let bids_bars = stock
                            .data_bids()
                            .iter()
                            .map(|(v, p)| {
                                Bar::new((p - stock.data_new()).into(), *v as f64).width(0.001)
                            })
                            .collect::<Vec<Bar>>();

                        let bid_chart = BarChart::new(bids_bars)
                            .allow_hover(false)
                            .color(Color32::LIGHT_GREEN);

                        let asks_bar: Vec<Bar> = stock
                            .data_asks()
                            .iter()
                            .map(|(v, p)| {
                                Bar::new((p - stock.data_new()).into(), *v as f64).width(0.001)
                            })
                            .collect();

                        let ask_chart = BarChart::new(asks_bar)
                            .allow_hover(false)
                            .color(Color32::YELLOW);
                        let plot = Plot::new(stock.code.to_string())
                            .allow_zoom(false)
                            .allow_drag(false)
                            .allow_scroll(false)
                            .show_grid([false, false])
                            .show_axes([false, false])
                            .sharp_grid_lines(false)
                            .width(50.0)
                            .height(16.0)
                            .center_x_axis(true)
                            .show(ui, |plot_ui| {
                                plot_ui.bar_chart(bid_chart);
                                plot_ui.bar_chart(ask_chart)
                            })
                            .response;

                        plot.on_hover_ui(|ui| {
                            ui.vertical(|ui| {
                                ui.group(|ui| {
                                    ui.set_max_size(Vec2::new(200.0, 120.0));
                                    let mut bids_text = vec![];
                                    let bids: Vec<Bar> = stock
                                        .data_bids()
                                        .iter()
                                        .map(|(v, p)| {
                                            bids_text.push(
                                                Text::new(
                                                    PlotPoint::new(-10.0, p - stock.data_new()),
                                                    format!(
                                                        "{:.2}    {}  -  {}  ",
                                                        (*v as f32) * (*p) * 0.01,
                                                        v,
                                                        p
                                                    ),
                                                )
                                                .anchor(Align2::RIGHT_CENTER),
                                            );
                                            Bar::new((p - stock.data_new()).into(), *v as f64)
                                                .width(0.001)
                                        })
                                        .collect();

                                    let mut asks_text = vec![];
                                    let asks: Vec<Bar> = stock
                                        .data_asks()
                                        .iter()
                                        .map(|(v, p)| {
                                            asks_text.push(
                                                Text::new(
                                                    PlotPoint::new(-10.0, p - stock.data_new()),
                                                    format!(
                                                        "{:.2}    {}  -  {}  ",
                                                        (*v as f32) * (*p) * 0.01,
                                                        v,
                                                        p
                                                    ),
                                                )
                                                .anchor(Align2::RIGHT_CENTER),
                                            );
                                            Bar::new((p - stock.data_new()).into(), *v as f64)
                                                .width(0.001)
                                        })
                                        .collect();

                                    let bid_chart =
                                        BarChart::new(bids).color(Color32::GREEN).horizontal();
                                    let ask_chart =
                                        BarChart::new(asks).color(Color32::YELLOW).horizontal();

                                    Plot::new(stock.code.to_string())
                                        .show_grid(false)
                                        .show_axes([false, false])
                                        .sharp_grid_lines(false)
                                        .show_background(false)
                                        .show_x(false)
                                        .show_y(true)
                                        .center_x_axis(true)
                                        .show(ui, |plot_ui| {
                                            plot_ui.bar_chart(bid_chart);
                                            plot_ui.bar_chart(ask_chart);

                                            plot_ui.hline(
                                                HLine::new(0.0)
                                                    .color(Color32::GRAY.linear_multiply(0.05)),
                                            );
                                            bids_text.iter().for_each(|t| {
                                                plot_ui.text(t.clone());
                                            });
                                            asks_text.iter().for_each(|t| {
                                                plot_ui.text(t.clone());
                                            })
                                        })
                                        .response;
                                });
                            });
                        });
                    });

                    ui.end_row();
                });
            });
    }

    fn _render_top_panel(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        // define a TopBottomPanel widget
        TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none())
            .show_inside(ui, |ui| {
                ui.add_space(2.0);

                menu::bar(ui, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                        ui.add_space(5.0);
                        let rocket_btn = ui
                            .add(Button::new(
                                RichText::new("🚀")
                                    .text_style(egui::TextStyle::Heading)
                                    .color(Color32::YELLOW),
                            ))
                            .on_hover_cursor(CursorIcon::Move);
                        if rocket_btn.is_pointer_button_down_on() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        };

                        ui.add(Label::new(
                            RichText::new("2025-02-01 17:05;55").text_style(egui::TextStyle::Small),
                        ));
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let close_btn = ui.add(Button::new(
                            RichText::new("❌")
                                .text_style(TextStyle::Body)
                                .color(Color32::RED),
                        ));
                        if close_btn.clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        let refresh_btn = ui.add(Button::new(
                            RichText::new("🔄")
                                .text_style(TextStyle::Body)
                                .color(Color32::GREEN),
                        ));
                        if refresh_btn.clicked() {}

                        // config button
                        let config_btn = ui.add(Button::new(
                            RichText::new("🛠")
                                .text_style(egui::TextStyle::Body)
                                .color(Color32::LIGHT_BLUE),
                        ));

                        if config_btn.clicked() {}
                    });
                });
                ui.add(Separator::default().spacing(0.0));
            });
    }

    fn refresh(&mut self) {
        self.stocks.iter_mut().for_each(|s| {
            let _ = s.get_klines(15, 50);
        });
    }
}
