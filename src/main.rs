use near_lake_framework::{
    near_indexer_primitives::StreamerMessage,
    LakeConfig,
};
use tokio::sync::mpsc;
use tracing_subscriber::EnvFilter;

fn main() {
    init_tracing();

    let config = LakeConfig {
        s3_config: None,
        s3_bucket_name: "near-lake-data-testnet".to_string(),
        s3_region_name: "eu-central-1".to_string(),
        start_block_height: 88630601,
    };

    let actix_rt = actix::System::new();
    actix_rt.block_on(async move {
        let (_, stream) = near_lake_framework::streamer(config);
        let _ = actix::spawn(handle_message_stream(stream)).await;

        tracing::debug!(target: "example", "Done");
    });
    actix_rt.run().expect("Actix runtime couldn't be run");

    tracing::debug!(target: "example", "Exiting");
}

async fn handle_message_stream(mut rx: mpsc::Receiver<StreamerMessage>) {
    let max_block_height = 88630610;
    while let Some(msg) = rx.recv().await {
        tracing::debug!(target: "example", "Got block: {}", msg.block.header.height);
        if msg.block.header.height >= max_block_height {
            return;
        }
    }
}

fn init_tracing() {
    let env_filter = EnvFilter::new("")
        .add_directive("example=debug".parse().unwrap())
        .add_directive("near_lake_framework=debug".parse().unwrap());

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stdout)
        .init();
}
