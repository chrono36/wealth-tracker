use crate::model::{
    stock::{KLineScale, Klines},
    Stock,
};

#[derive(Debug)]
pub enum StockCammnd {
    Refresh, //refetch data
    SetInterval(u32),
    StockAdd(String),
    StockDel(String),
    StockKLine(String, KLineScale),
}

// send data to view

#[derive(Debug)]
pub enum TxStockData {
    Stock(Stock),
    StockList(Vec<Stock>),
    Kline((String, Klines)),
}
