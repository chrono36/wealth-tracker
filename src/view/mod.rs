use std::{collections::HashMap, thread};

use crossbeam::channel::{Receiver, Sender};
use eframe::CreationContext;
use egui::{
    menu, Align, Align2, Button, CollapsingHeader, Color32, Context, CursorIcon, Grid, Label,
    Layout, RichText, Separator, Shadow, SidePanel, Slider, Stroke, TextStyle, TopBottomPanel,
    Vec2,
};
use egui_plot::{
    AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, HLine, Plot, PlotPoint, Text,
};
use serde::{Deserialize, Serialize};

use crate::{
    backend::{StockCammnd, StockTask, TxStockData},
    model::{stock, Stock},
};

#[derive(Default)]
pub struct StockTrackerView {
    data: HashMap<String, Stock>,
    show_klines_viewport: bool,
    setting: Setting,
    tx: Option<Sender<StockCammnd>>,
    rx: Option<Receiver<TxStockData>>,
    time: String,
}

#[derive(Default, Serialize, Deserialize)]
struct Setting {
    open: bool,
    show_name: bool,
    show_color: bool,
    interval: u32,
    stocks: String,
    adding_code: String,
}

impl StockTrackerView {
    fn name(&self) -> &'static str {
        "Stock Tracker"
    }

    pub fn new(cc: &CreationContext) -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();
        let (tx2, rx2) = crossbeam::channel::unbounded();

        let mut app = StockTrackerView::default();

        if let Some(storage) = cc.storage {
            if let Some(setting) = eframe::get_value(storage, eframe::APP_KEY) {
                app.setting = setting
            }
        }
        let mut codes = app.setting.stocks.clone();

        // 默认三大指数
        if codes.is_empty() {
            codes = String::from("sh000001,sz399001,sh000300");
        }
        thread::spawn(|| StockTask::new(rx, tx2, codes).run());
        app.tx = Some(tx);
        app.rx = Some(rx2);
        app
    }
}

impl StockTrackerView {
    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        let frame = egui::Frame::none().shadow(Shadow::NONE);
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            // .frame(frame)
            .open(open)
            .resizable([true, true])
            .scroll(true)
            .show(ctx, |ui| {
                ctx.request_repaint();
                self._render_top_panel(ctx, ui);
                self.receiver();
                self.render_stocks(ctx, ui);
                // self.render_setting(ctx);
            });
    }

    fn receiver(&mut self) {
        if let Some(rx) = &self.rx {
            match rx.try_recv() {
                Ok(data) => match data {
                    TxStockData::Stock(stock) => {
                        if let Some(s) = self.data.get_mut(&stock.code) {
                            s.data = stock.data.clone();
                        } else {
                            self.data.insert(stock.code.clone(), stock);
                        }
                    }
                    TxStockData::StockList(stocks) => {
                        self.update_time();
                        stocks.iter().for_each(|stock| {
                            if let Some(s) = self.data.get_mut(&stock.code) {
                                s.data = stock.data.clone();
                            } else {
                                self.data.insert(stock.code.to_string(), stock.clone());
                            }
                        });
                    }
                    TxStockData::Kline((code, kline)) => {
                        //
                        if let Some(s) = self.data.get_mut(&code) {
                            s.kline = kline;
                        }
                    }
                },
                Err(e) => {
                    let _ = e;
                }
            }
        }
    }
}

impl StockTrackerView {
    fn render_stocks(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.add_space(2.0);

        Grid::new("Stock list")
            .max_col_width(60.0)
            .striped(true)
            .show(ui, |ui| {
                self.data.values_mut().for_each(|stock| {
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

                    // amount
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
                                                    if let Some(tx) = &self.tx {
                                                        let _ = tx.send(StockCammnd::StockKLine(
                                                            stock.code.to_string(),
                                                            stock::KLineScale::Munute5,
                                                        ));
                                                    }
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
                                                    if let Some(tx) = &self.tx {
                                                        let _ = tx.send(StockCammnd::StockKLine(
                                                            stock.code.to_string(),
                                                            stock::KLineScale::Munute15,
                                                        ));
                                                    }
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
                                                    if let Some(tx) = &self.tx {
                                                        let _ = tx.send(StockCammnd::StockKLine(
                                                            stock.code.to_string(),
                                                            stock::KLineScale::Munute30,
                                                        ));
                                                    }
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
                                                    if let Some(tx) = &self.tx {
                                                        let _ = tx.send(StockCammnd::StockKLine(
                                                            stock.code.to_string(),
                                                            stock::KLineScale::Day,
                                                        ));
                                                    }
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
                                                        // x.day.and_utc().timestamp() as f64,
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
                                                // .custom_x_axes(vec![
                                                //     AxisHints::new_x().label("Time (s)"),
                                                //     AxisHints::new_x().label("Time").formatter(
                                                //         |x, _| {
                                                //             let date_time =
                                                //                 DateTime::from_timestamp(
                                                //                     x.value as i64,
                                                //                     0,
                                                //                 )
                                                //                 .unwrap();
                                                //             date_time
                                                //                 .format("%Y-%m-%d %H:%M:%S")
                                                //                 .to_string()
                                                //         },
                                                //     ),
                                                // ])
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
            // .frame(egui::Frame::none())
            .show_inside(ui, |ui| {
                ui.add_space(2.0);

                menu::bar(ui, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                        ui.add_space(5.0);
                        let rocket_btn = ui
                            .add(Button::new(
                                RichText::new("🚀").text_style(egui::TextStyle::Heading), // .color(Color32::YELLOW)
                            ))
                            .on_hover_cursor(CursorIcon::Move);
                        if rocket_btn.is_pointer_button_down_on() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        };

                        ui.add(Label::new(
                            RichText::new(self.time.clone()).text_style(egui::TextStyle::Small),
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

    fn render_setting(&mut self, ctx: &eframe::egui::Context) {
        if self.setting.open {
            SidePanel::right("setting").show(ctx, |ui| {
                menu::bar(ui, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.label(RichText::new("⚙ setting").color(Color32::LIGHT_BLUE));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let close_btn = ui.add(Button::new(
                                RichText::new("\u{2bab}")
                                    .text_style(TextStyle::Body)
                                    .color(Color32::GRAY),
                            ));
                            if close_btn.clicked() {
                                self.setting.open = false
                            }
                        });
                    });
                    self.render_setting_content(ui);
                });
            });
        }
    }

    fn render_setting_content(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().slider_width = 50.0;
            ui.label(RichText::new("🕘").color(Color32::GREEN));
            let interval_slider = ui.add(
                Slider::new(&mut self.setting.interval, 200..=1000)
                    .suffix(" ms")
                    .step_by(100.0),
            );
            // set interval
            if interval_slider.changed() {
                if let Some(tx) = &self.tx {
                    let _ = tx.send(StockCammnd::SetInterval(self.setting.interval));
                }
            }
            ui.add(Separator::default().spacing(0.0));

            // stocks list
            ui.horizontal(|ui| {
                //
                ui.label(RichText::new("📓").color(Color32::LIGHT_BLUE));
                CollapsingHeader::new("stocks")
                    .default_open(false)
                    .show(ui, |ui| {
                        for (_code, s) in self.data.clone() {
                            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("{}({})", s.name, s.code))
                                        .color(Color32::LIGHT_BLUE),
                                );
                                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                    ui.add_space(3.0);
                                    let close_btn = ui.add(Button::new(
                                        RichText::new("❌")
                                            .text_style(TextStyle::Body)
                                            .color(Color32::RED),
                                    ));
                                    if close_btn.clicked() {
                                        self.data.remove(&s.code);
                                    }
                                });
                            });
                        }
                    });
            });
            ui.add(Separator::default().spacing(0.0));
            ui.horizontal(|ui| {
                // add stock code
            });
        });
    }

    fn update_time(&mut self) {
        self.time = format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    }
}
