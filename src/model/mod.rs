use stock::Klines;

pub mod stock;

#[derive(Clone, Default, Debug)]
pub struct Stock {
    pub name: String,
    pub code: String,
    pub data: StockData,
    pub kline: Klines,
}

impl Stock {
    #[inline]
    pub fn data_new(&self) -> Price {
        self.data.new
    }

    #[inline]
    pub fn data_rise_per(&self) -> Price {
        self.data.rise_per
    }

    #[inline]
    pub fn data_bids(&self) -> &Vec<(Vol, Price)> {
        &self.data.bids
    }

    #[inline]
    pub fn data_asks(&self) -> &Vec<(Vol, Price)> {
        &self.data.asks
    }
}

pub type Vol = u64;
pub type Price = f32;

#[derive(Clone, Default, Debug)]
pub struct StockData {
    pub date: String,
    pub time: String,
    pub opening: Price,
    pub closing: Price,
    pub hight: Price,
    pub low: Price,
    pub vol: Vol,
    pub amount: f32,
    pub bid: Price,
    pub ask: Price,
    pub new: Price,
    pub rise_per: f32,
    pub bids: Vec<(Vol, Price)>,
    pub asks: Vec<(Vol, Price)>,
}
