use std::sync::{
    atomic::{AtomicU16, Ordering},
    Once,
};
use tracing_subscriber::EnvFilter;

static PORT: AtomicU16 = AtomicU16::new(17000);

pub fn get_port() -> u16 {
    PORT.fetch_add(1, Ordering::SeqCst)
}

pub fn start_global_subscriber() {
    static START_GLOBAL_SUBSCRIBER: Once = Once::new();

    START_GLOBAL_SUBSCRIBER.call_once(|| {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    });
}
