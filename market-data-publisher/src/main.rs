pub mod producer;
mod utils;

use env_logger::Env;

use tokio::signal;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::available_parallelism;

const MARKET_DATA_SUBJECT: &str = "market-data";

const DEFAULT_BATCH_SIZE: i8 = 50;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Get the number of available CPU cores
    let num_cores = available_parallelism().unwrap().get();

    log::info!("Number of CPU cores: {}", num_cores);

    // Introduce a flag for graceful termination
    let should_terminate = Arc::new(Mutex::new(false));

    let tasks = (0..num_cores)
        .map(|_| {
            // Clone a reference to should_terminate for the stream
            let should_terminate_signal = Arc::clone(&should_terminate);
            tokio::spawn(async move { producer::load(should_terminate_signal).await })
        })
        .collect::<Vec<_>>();

    signal::ctrl_c().await.expect("failed to listen for event");

    log::warn!("Shutting down gracefully...");

    // Update should_terminate when Ctrl+C is received
    *should_terminate.lock().unwrap() = true;

    let mut task_counters = 0;

    // Wait for all tasks to complete and collect their results
    for handle in tasks {
        let val = handle.await.expect("Failed to await task");
        task_counters += val;
    }

    log::info!(
        "Total records produced: {}",
        task_counters * (DEFAULT_BATCH_SIZE as i32)
    );

    log::warn!("Graceful shutdown completed!");

    Ok(())
}
