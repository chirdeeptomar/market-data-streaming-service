use shared::models::MarketData;
use std::sync::{Arc, Mutex};

type ThreadsafeSerializer =
    Arc<Mutex<fn(MarketData) -> Result<Vec<u8>, shared::serializer::MessageConversionError>>>;

pub fn genrate_message(thread_serializer: &ThreadsafeSerializer) -> Vec<u8> {
    // Access the serializer within the thread using a Mutex guard
    let serialize = thread_serializer.lock().unwrap();

    let new_order = MarketData::new();

    serialize(new_order)
        .map_err(|e| log::error!("Error Serializing message: {}", e.message))
        .unwrap()
}
