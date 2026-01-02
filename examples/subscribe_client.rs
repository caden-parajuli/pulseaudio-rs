use std::time::Duration;

use pulseaudio::protocol::{self, SubscriptionEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find and connect to PulseAudio. The socket is usually in a well-known
    // location under XDG_RUNTIME_DIR.
    let client = pulseaudio::Client::from_env(c"subscribe_client")?;

    client
        .subscribe(
            protocol::SubscriptionMask::ALL,
            Box::new(|event: SubscriptionEvent| {
                eprintln!(
                    "got event {:?} for ID {:?} ({:?})",
                    event.event_type, event.index, event.event_facility
                )
            }),
        )
        .await?;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
