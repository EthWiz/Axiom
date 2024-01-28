#[derive(Debug, Clone)]
pub struct Stock {
    pub ticker: String,
}
#[derive(Debug, Clone)]
pub struct StockHeld {
    pub stock: Stock,
    pub cost_basis: f64,
    pub number_of_shares: i32,
}
