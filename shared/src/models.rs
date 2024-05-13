use chrono::{DateTime, Utc};
use fake::{faker::finance::en::Isin, Fake};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::RandomEnum;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AssetClass {
    FX,
    FI,
    COMMODITY,
    EQUITY,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Side {
    B,
    S,
}

impl RandomEnum for AssetClass {
    fn variants() -> &'static [AssetClass] {
        &[
            AssetClass::FI,
            AssetClass::FX,
            AssetClass::COMMODITY,
            AssetClass::EQUITY,
        ]
    }
}

impl RandomEnum for Side {
    fn variants() -> &'static [Side] {
        &[Side::B, Side::S]
    }
}

#[derive(Serialize, Deserialize)]
pub struct MarketData {
    pub id: Uuid,
    pub ask: f64,
    pub bid: f64,
    pub timestamp: DateTime<Utc>,
    pub instrument: String,
    pub asset_class: AssetClass,
    pub side: Side,
    pub volume: i64,
}

impl MarketData {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            instrument: Isin().fake(),
            asset_class: AssetClass::random(),
            side: Side::random(),
            volume: rand::thread_rng().gen(),
            ask: rand::thread_rng().gen(),
            bid: rand::thread_rng().gen(),
        }
    }
}

impl Default for MarketData {
    fn default() -> Self {
        Self::new()
    }
}
