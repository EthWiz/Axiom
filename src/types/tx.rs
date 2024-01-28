use crate::types::stock::Stock;
pub struct Tx {
    tx_id: String,
    date: String,
    number_of_shares: i32,
    price_per_stock: f64,
    value: f64,
    stock: Stock,
}
