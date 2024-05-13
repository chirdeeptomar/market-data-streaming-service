use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};

use shared::{models::MarketData, nats::get_jetstream_context};

use crate::{utils, MARKET_DATA_SUBJECT};

pub(crate) async fn load(should_terminate: Arc<Mutex<bool>>) -> i32 {
    let args = utils::args_parser::parse(env::args().collect());

    let serializer = shared::serializer_factory::get_serializer::<MarketData>(args.serializer_type);

    // Wrap it in Arc<Mutex<>> for thread safety
    let wrapped_serializer = Arc::new(Mutex::new(serializer));

    // Spawn a thread and move the serializer into it
    let cloned_serializer = Arc::clone(&wrapped_serializer);

    log::info!("Connecting to Messaging System...");

    let client = get_jetstream_context().await;

    log::info!("Connection Established...");

    log::info!("Sending messages started. ");

    let mut counter = 0;

    while !*should_terminate.lock().unwrap() {
        let batch = utils::data_factory::genrate_message(&cloned_serializer);

        let result = client.publish(MARKET_DATA_SUBJECT, batch.into()).await;

        match result {
            Ok(_) => {
                counter += 1;
                log::info!("Batch Saved: {:?}:{}", thread::current().id(), counter);
            }
            Err(err) => {
                log::error!("Error occured");
                log::error!("{}", err)
            }
        }
    }

    counter
}
