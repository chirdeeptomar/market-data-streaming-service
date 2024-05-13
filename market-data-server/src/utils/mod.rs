pub mod args_parser;
pub mod nats_consumer;

use crate::proto::{MarketDataPayload, MarketDataRequest};
use chrono::Utc;
use fake::faker::finance::en::Isin;
use fake::Fake;
use rand::{thread_rng, Rng};
use uuid::Uuid;

pub(crate) trait Factory<T> {
    fn new(t: &T) -> Self;
}

// Implement the Iterator trait for MarketDataPayload
impl Iterator for MarketDataPayload {
    type Item = MarketDataPayload;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = thread_rng();
        Some(Self::Item {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            instrument: Isin().fake(),
            asset_class: rng.gen_range(0..3),
            side: rng.gen_range(0..1),
            volume: rand::thread_rng().gen(),
            ask: rand::thread_rng().gen(),
            bid: rand::thread_rng().gen(),
        })
    }
}

impl Factory<MarketDataRequest> for MarketDataPayload {
    fn new(input: &MarketDataRequest) -> Self {
        let mut rng = thread_rng();
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            instrument: Isin().fake(),
            asset_class: rng.gen_range(0..3),
            side: rng.gen_range(0..1),
            volume: rand::thread_rng().gen(),
            ask: rand::thread_rng().gen(),
            bid: rand::thread_rng().gen(),
        }
    }
}
