use subxt::{OnlineClient, SubstrateConfig};

#[subxt::subxt(runtime_metadata_path = "shiden-metadata.scale")]
pub mod runtime {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://shiden.public.blastapi.io:443";
    let api = OnlineClient::<SubstrateConfig>::from_url(url).await?;

    let address = runtime::constants().balances().existential_deposit();
    let existential_deposit = api.constants().at(&address)?;
    println!("Existential Deposit: {}", existential_deposit);

    let active_era_addr = runtime::storage().dapps_staking().current_era();
    let era = api.storage().fetch(&active_era_addr, None).await?.unwrap();
    println!("Current era: {:?}", era);

    Ok(())
}
