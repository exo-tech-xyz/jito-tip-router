use ::{
    anyhow::Result,
    clap::Parser,
    ellipsis_client::EllipsisClient,
    log::{error, info},
    solana_rpc_client::rpc_client::RpcClient,
    solana_sdk::signer::keypair::read_keypair_file,
    tip_router_operator_cli::{
        cli::{Cli, Commands},
        process_epoch::{get_previous_epoch_last_slot, process_epoch, wait_for_next_epoch},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    let keypair = read_keypair_file(&cli.keypair_path).expect("Failed to read keypair file");
    let rpc_client = EllipsisClient::from_rpc(
        RpcClient::new(cli.rpc_url.clone()),
        &read_keypair_file(&cli.keypair_path).expect("Failed to read keypair file"),
    )?;

    match &cli.command {
        Commands::Monitor {
            ncn_address,
            tip_distribution_program_id,
            tip_payment_program_id,
        } => {
            info!("Starting epoch monitor...");

            loop {
                // Wait for epoch change
                wait_for_next_epoch(&rpc_client).await?;

                // Get the last slot of the previous epoch
                let previous_epoch_slot = get_previous_epoch_last_slot(&rpc_client).await?;
                info!("Processing slot {} for previous epoch", previous_epoch_slot);

                // TODO Process the epoch

                // match process_epoch(
                //     previous_epoch_slot,
                //     &cli,
                //     &keypair,
                //     tip_distribution_program_id,
                //     tip_payment_program_id,
                //     ncn_address,
                // )
                // .await
                // {
                //     Ok(_) => info!("Successfully processed epoch"),
                //     Err(e) => {
                //         error!("Error processing epoch: {}", e);
                //         // Continue to next epoch even if this one failed
                //     }
                // }
            }
        }
    }
}
