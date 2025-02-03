pub mod bitcoin_view;
pub mod setting_view;
pub mod stock_setting;
pub mod stocks;

pub use bitcoin_view::*;
pub use stocks::*;

use serde::{Deserialize, Serialize};

use crate::model::stock;

#[derive(Default, Serialize, Deserialize)]
struct Setting {
    open: bool,
    show_name: bool,
    show_color: bool,
    interval: u32,
    stocks: String,
    adding_code: String,
}
