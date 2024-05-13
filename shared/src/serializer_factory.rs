use crate::serializer::{
    deserialize_from_json, deserialize_from_messagepack, serialize_to_json,
    serialize_to_messagepack, MessageConversionError, SerializationType,
};
use async_nats::jetstream::Message;
use serde::{Deserialize, Serialize};

type FuncTypeToVec<T> = fn(T) -> Result<Vec<u8>, MessageConversionError>;
type FuncMessageToType<'a, T> = fn(&'a Message) -> Result<T, MessageConversionError>;

// Factory function to create a serializer based on type
pub fn get_serializer<T: Serialize>(serializer_type: SerializationType) -> FuncTypeToVec<T> {
    match serializer_type {
        SerializationType::Json => serialize_to_json,
        SerializationType::Messagepack => serialize_to_messagepack,
    }
}

// Factory function to create a de-serializer based on type
pub fn get_deserializer<'a, T: Deserialize<'a>>(
    serializer_type: SerializationType,
) -> FuncMessageToType<'a, T> {
    match serializer_type {
        SerializationType::Json => deserialize_from_json,
        SerializationType::Messagepack => deserialize_from_messagepack,
    }
}
