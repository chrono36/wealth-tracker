use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use serde_json::Value;

use crate::error::TrackerError;

use super::{Price, Stock, StockData, Vol};

const BASE_URL: &str = "http://hq.sinajs.cn";
const MIN_LEN: usize = "var hq_str_cc000000=\"\";".len();

#[derive(Default, Debug, Clone)]
pub struct KlineItem {
    pub day: NaiveDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub amount: f64,
    pub date: String,
}

#[derive(Debug, Clone, Default)]
pub struct Klines {
    pub scale: KLineScale,
    pub klines: Vec<KlineItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum KLineScale {
    Munute5,
    #[default]
    Munute15,
    Munute30,
    Hour,
    Day,
    Week,
    Month,
}

impl KLineScale {
    pub fn to_usize(&self) -> usize {
        match self {
            KLineScale::Munute5 => 5,
            KLineScale::Munute15 => 15,
            KLineScale::Munute30 => 30,
            KLineScale::Hour => 60,
            KLineScale::Day => 240,
            KLineScale::Week => 1200,
            KLineScale::Month => 7200,
        }
    }
}

impl From<usize> for KLineScale {
    fn from(value: usize) -> Self {
        match value {
            5 => KLineScale::Munute5,
            15 => KLineScale::Munute15,
            30 => KLineScale::Munute30,
            60 => KLineScale::Hour,
            240 => KLineScale::Day,
            1200 => KLineScale::Week,
            7200 => KLineScale::Month,
            _ => KLineScale::Munute15,
        }
    }
}

impl From<Value> for KlineItem {
    fn from(value: Value) -> Self {
        let day_str = value["day"].as_str().unwrap();

        let date = match NaiveDateTime::parse_from_str(day_str, "%Y-%m-%d %H:%M:%S") {
            Ok(d) => d,
            Err(_) => NaiveDate::parse_from_str(day_str, "%Y-%m-%d")
                .unwrap()
                .into(),
        };

        Self {
            day: date,
            open: f64::from_str(value["open"].as_str().unwrap_or("0")).unwrap(),
            high: f64::from_str(value["high"].as_str().unwrap_or("0")).unwrap(),
            low: f64::from_str(value["low"].as_str().unwrap_or("0")).unwrap(),
            close: f64::from_str(value["close"].as_str().unwrap_or("0")).unwrap(),
            volume: f64::from_str(value["volume"].as_str().unwrap_or("0")).unwrap(),
            amount: f64::from_str(value["amount"].as_str().unwrap_or("0")).unwrap(),
            date: day_str.to_string(),
        }
    }
}

impl Klines {
    pub fn get_klines(code: &str, scale: usize, datalen: u32) -> Result<Klines, TrackerError> {
        let response = reqwest::blocking::get(format!("https://quotes.sina.cn/cn/api/json_v2.php/CN_MarketDataService.getKLineData?symbol={code}&scale={scale}&ma=no&datalen={datalen}"))?.json::<Vec<Value>>()?;

        let klines = response
            .into_iter()
            .map(KlineItem::from)
            .collect::<Vec<KlineItem>>();

        let kline = Klines {
            klines: klines,
            scale: KLineScale::from(scale),
        };

        Ok(kline)
    }
}

impl Stock {
    pub fn get_klines(&mut self, scale: usize, datalen: u32) -> Result<(), TrackerError> {
        let code = self.code.as_str();

        let kline_scale = KLineScale::from(scale);

        if self.kline.scale == kline_scale && !self.kline.klines.is_empty() {
            return Ok(());
        }

        let result = Klines::get_klines(code, scale, datalen);
        match result {
            Ok(klines) => self.kline = klines,
            Err(_e) => {}
        }

        Ok(())
    }
}

pub fn fetch_data_list(codes: Vec<String>) -> Result<Vec<Stock>, TrackerError> {
    let code_string = codes.join(",");
    let url = format!("{}/list={}", BASE_URL, code_string);

    // var hq_str_sh601127 = "赛力斯,133.000,132.800,132.790,135.440,131.010,132.790,132.800,22615984,3006594293.000,25300,132.790,31600,132.780,16400,132.770,9800,132.760,8600,132.750,64500,132.800,16900,132.810,11900,132.820,1000,132.830,1900,132.840,2025-01-27,15:00:01,00,";
    // "var hq_str_s_sh000001=\"上证指数,3250.6007,-2.0257,-0.06,3874676,45023154\";\nvar hq_str_s_sz399001=\"深证成指,10156.07,-136.663,-1.33,532847417,66764113\";\nvar hq_str_s_sh000300=\"沪深300,3817.0802,-15.7835,-0.41,1466707,28294013\";\n"
    let str = reqwest::blocking::Client::new()
        .get(&url)
        .header("Referer", "https://www.sina.com.cn/")
        .send()?
        .text()?;

    let stocks: Vec<Stock> = str
        .trim()
        .split('\n')
        .filter(|x| x.len() > MIN_LEN)
        .filter_map(|stock_item_str| decode_sina_result(stock_item_str))
        .collect();

    Ok(stocks)
}

fn decode_sina_result(stock_string: &str) -> Option<Stock> {
    let mut list: Vec<&str> = stock_string.trim().split(",").collect();
    // ["var hq_str_sh601127=\"赛力斯", "133.000", "132.800", "132.790", "135.440", "131.010", "132.790", "132.800", "22615984", "3006594293.000", "25300", "132.790", "31600", "132.780", "16400", "132.770", "9800", "132.760", "8600", "132.750", "64500", "132.800", "16900", "132.810", "11900", "132.820", "1000", "132.830", "1900", "132.840", "2025-01-27", "15:00:01", "00", "\";"]
    list.truncate(32);
    match list.as_slice() {
        [code_and_name, opening_str, closing_str, new_str, high, low, bid, ask, vol, amount, rest @ .., date, time] =>
        {
            let name_str: Vec<&str> = code_and_name.split("=\"").collect();
            let code = name_str[0].replace("var hq_str_", "");
            let name = name_str[1];
            let opening = opening_str.parse::<Price>().unwrap();
            let closing = closing_str.parse::<Price>().unwrap();
            let new = new_str.parse::<Price>().unwrap();
            let percent = ((new - closing) / closing * 10000.0).round() / 100.0;

            let bids = rest[0..10]
                .chunks(2)
                .into_iter()
                .map(|x| {
                    if let [v, p] = x {
                        (v.parse::<Vol>().unwrap() / 100, p.parse::<Price>().unwrap())
                    } else {
                        (0, 0.0)
                    }
                })
                .collect();

            let asks = rest[10..20]
                .chunks(2)
                .into_iter()
                .map(|x| {
                    if let [v, p] = x {
                        (v.parse::<Vol>().unwrap() / 100, p.parse::<Price>().unwrap())
                    } else {
                        (0, 0.0)
                    }
                })
                .collect();

            let data = StockData {
                opening,
                closing,
                new,
                hight: high.parse::<Price>().unwrap(),
                low: low.parse::<Price>().unwrap(),
                bid: bid.parse::<Price>().unwrap(),
                ask: ask.parse::<Price>().unwrap(),
                vol: vol.parse::<Vol>().unwrap(),
                amount: amount.parse::<f32>().unwrap(),
                date: date.to_string(),
                time: time.to_string(),
                rise_per: percent,
                bids,
                asks,
            };

            Some(Stock {
                name: name.into(),
                code: code.into(),
                data,
                ..Default::default()
            })
        }

        _ => None,
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_fetch_data() {
        let codes = vec![
            "sh000001".to_string(),
            "sz399001".to_string(),
            "sh000300".to_string(),
            "sh601127".to_string(),
        ];

        let res = fetch_data_list(codes);

        match res {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn test_get_klines() {
        let mut stock = Stock::default();
        stock.code = String::from("sh601127");

        let _ = stock.get_klines(5, 50);

        println!("{:?}", stock.kline);
    }
}
