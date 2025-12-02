pub struct Deal {
    pub quantity: f64,
    pub sell_stop_price: f64,
    pub sell_price: f64,
    pub buy_price: f64,
    pub buy_stop_price: f64,
}

impl Deal {
    pub fn new(quantity: f64, base_price: f64, padding_percent: f64, stop_percent: f64) -> Self {
        let padding_size = base_price * (padding_percent / 100.0);
        let stop_padding_size = base_price * (stop_percent / 100.0);
        let sell_stop_price = base_price + stop_padding_size;
        let sell_price = base_price + padding_size;
        let buy_price = base_price - padding_size;
        let buy_stop_price = base_price - stop_padding_size;

        Self {
            quantity,
            sell_stop_price,
            sell_price,
            buy_price,
            buy_stop_price,
        }
    }
}
