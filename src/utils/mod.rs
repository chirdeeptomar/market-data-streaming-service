use crate::proto::{MarketDataPayload, MarketDataRequest};
use rand::{thread_rng, Rng};

pub(crate) trait Factory<T> {
    fn new(t: &T) -> Self;
}

impl Factory<MarketDataRequest> for MarketDataPayload {
    fn new(input: &MarketDataRequest) -> Self {
        Self {
            instrument: input.instrument.to_string(),
            bid: thread_rng().gen(),
            ask: thread_rng().gen(),
            bid_size: None,
            ask_size: None,
            last_sale: None,
            last_size: None,
            quote_time: None,
            trade_time: None,
            exchange: None,
            volume: None,
        }
    }
}
