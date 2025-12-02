use crate::candle::Candle;
use crate::simulator::deal::Deal;

pub enum Side {
    None,
    Long,
    Short,
}

pub struct Position {
    side: Side,
    waited_deals: Vec<Deal>,
    opened_deals: Vec<Deal>,
    middle_price: f64,
    liquidation_price: f64,
    capital: f64,
}

impl Position {
    pub fn new() -> Self {
        Self {
            side: Side::None,
            waited_deals: Vec::new(),
            opened_deals: Vec::new(),
            middle_price: 0.0,
            liquidation_price: 0.0,
            capital: 100.0,
        }
    }

    pub fn handle_deal(&mut self, deal: Deal, candle: &Candle) {
        if candle.high > deal.sell_price && candle.high < deal.buy_price {
            // TODO Сразу профит на основе размера квантити
        } else if candle.high > deal.sell_price {
            // TODO Продали
        } else if candle.low < deal.buy_price {
            // TODO Купили
        } else {
            // TODO Оно застряло внутри
        }

        // TODO Нужно учитывать цену ликвидации с учетом куда ходил хай и лой,
        // ну и учитывать какая сейчас позиция
    }

    // TODO -
    // TODO Каждую неделю реинвестировать прибыль?
}
