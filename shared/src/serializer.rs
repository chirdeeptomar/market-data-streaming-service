use async_nats::jetstream::Message;
use serde::{Deserialize, Serialize};

pub enum SerializationType {
    Json,
    Messagepack,
}

pub struct MessageConversionError {
    pub message: String,
}

pub fn serialize_to_json<T: Serialize>(item: T) -> Result<Vec<u8>, MessageConversionError> {
    serde_json::to_vec(&item).map_err(|err| MessageConversionError {
        message: err.to_string(),
    })
}

pub fn serialize_to_messagepack<T: Serialize>(item: T) -> Result<Vec<u8>, MessageConversionError> {
    rmp_serde::to_vec(&item).map_err(|err| MessageConversionError {
        message: err.to_string(),
    })
}

pub fn deserialize_from_json<'a, T: Deserialize<'a>>(
    message: &'a Message,
) -> Result<T, MessageConversionError> {
    serde_json::from_slice::<T>(message.payload.as_ref()).map_err(|err| MessageConversionError {
        message: err.to_string(),
    })
}

pub fn deserialize_from_messagepack<'a, T: Deserialize<'a>>(
    message: &'a Message,
) -> Result<T, MessageConversionError> {
    rmp_serde::from_slice::<T>(message.payload.as_ref()).map_err(|err| MessageConversionError {
        message: err.to_string(),
    })
}
