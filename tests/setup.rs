use tracing::{info, Level};
use tracing_subscriber;
use tracing_subscriber::FmtSubscriber;

pub fn setup() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}