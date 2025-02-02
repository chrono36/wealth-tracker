// excute task

pub mod message;
use std::{collections::HashMap, time::Duration};

use crossbeam::channel::{tick, Receiver, Sender};
use crossbeam::select;
pub use message::*;

use crate::model::{
    stock::{self, KLineScale, Klines},
    Stock,
};

#[derive(Debug, Clone)]
pub struct StockTask {
    stock_codes: Vec<String>,
    kline_scale_map: HashMap<String, KLineScale>,
    rx: Receiver<StockCammnd>,
    tx: Sender<TxStockData>,
}

impl StockTask {
    pub fn new(rx: Receiver<StockCammnd>, tx: Sender<TxStockData>, codes: String) -> Self {
        let stock_codes = codes
            .split(",")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Self {
            stock_codes: stock_codes,
            rx: rx,
            tx: tx,
            kline_scale_map: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        self.refresh_data();
        self.refresh_klines();
        let mut ticker = tick(Duration::from_millis(200));
        let kline_ticker = tick(Duration::from_secs(60));
        loop {
            select! {
                recv(self.rx) -> msg => {
                    match msg {
                        Ok(m) => {
                            match m {
                                StockCammnd::Refresh =>{
                                    self.refresh_data();
                                },
                                StockCammnd::SetInterval(interval) => {
                                    ticker = tick(Duration::from_secs(interval.into()));
                                },
                                StockCammnd::StockAdd(code) => {
                                    self.add_code(code);
                                },
                                StockCammnd::StockDel(code) => {
                                    self.remove_code(code.as_str());
                                    //
                                },
                                StockCammnd::StockKLine(code, scale) => {
                                    let scale_int = scale.to_usize();
                                    self.kline_scale_map.insert(code.clone(), scale);
                                    // get kline
                                    let _ = match Klines::get_klines(&code, scale_int, 100) {
                                        Ok(klines) => {
                                            self.tx.send(TxStockData::Kline((code,klines))).ok();
                                        } ,
                                        Err(e) => {
                                            println!("kline error {}",e);
                                        },
                                    };

                                },
                            }
                        },
                        Err(e) => {

                        },
                    }
                }  ,
                recv(ticker)->_msg =>{
                    self.refresh_data();
                },
                recv(kline_ticker) -> _msg => {
                    self.refresh_klines();
                }
            }
        }
    }

    fn refresh_data(&self) {
        if !self.stock_codes.is_empty() {
            match stock::fetch_data_list(self.stock_codes.clone()) {
                Ok(v) => {
                    // refresh data
                    self.tx.send(TxStockData::StockList(v)).ok();
                }
                Err(_) => {}
            }
        }
    }

    fn refresh_klines(&self) {
        if !self.stock_codes.is_empty() {
            self.stock_codes.iter().for_each(|code| {
                // fetch kline data
                let mut s = Stock::default();
                s.code = code.clone();
                let scale = self
                    .kline_scale_map
                    .get(code)
                    .unwrap_or(&KLineScale::Munute15);
                match Klines::get_klines(code, scale.to_usize(), 100) {
                    Ok(klines) => {
                        let data = TxStockData::Kline((code.clone(), klines));
                        self.tx.send(data).ok()
                    }
                    Err(_) => todo!(),
                };
            });
            //x
        }
    }

    fn add_code(&mut self, code: String) {
        // check code illege

        if self.stock_codes.contains(&code) {
            self.stock_codes.push(code);
        }
    }

    fn remove_code(&mut self, code: &str) {
        self.stock_codes.retain(|x| x != code);
    }
}
