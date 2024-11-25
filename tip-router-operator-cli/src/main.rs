use clap::Parser;
use anyhow::Result;
use log::info;
use snapshot::SnapshotCreator;
use solana_metrics::datapoint_info;
use solana_metrics::datapoint_error;

mod snapshot;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to operator keypair
    #[arg(short, long)]
    keypair_path: String,

    /// RPC URL
    #[arg(short, long, default_value = "http://localhost:8899")]
    rpc_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Start monitoring tips
    Monitor {
        /// NCN address
        #[arg(short, long)]
        ncn_address: String,
    },
    /// Create and validate snapshots at epoch boundaries
    Snapshot {
        /// Output directory for snapshots
        #[arg(short, long)]
        output_dir: String,

        /// Maximum number of snapshots to retain
        #[arg(short, long, default_value = "2")]
        max_snapshots: u32,

        /// Snapshot compression type (none, bzip2, gzip, zstd)
        #[arg(short, long, default_value = "zstd")]
        compression: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { ncn_address } => {
            info!("Starting monitor for NCN address: {}", ncn_address);
            info!("Using keypair at: {}", cli.keypair_path);
            info!("Connected to RPC: {}", cli.rpc_url);
            // TODO: Implement monitoring logic
        }
        Commands::Snapshot { output_dir, max_snapshots, compression } => {
            info!("Starting snapshot creator");
            info!("Output directory: {}", output_dir);
            let snapshot_creator = SnapshotCreator::new(
                &cli.keypair_path,  // Add keypair path
                &cli.rpc_url,
                &output_dir,        // Add & to make it a reference
                max_snapshots,
                compression,
                true,              // Add is_archive parameter
            )?;
            snapshot_creator.monitor_epoch_boundary().await?;
        }
    }

    Ok(())
}
