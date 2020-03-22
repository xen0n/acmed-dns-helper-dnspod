use log::info;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "acmed-dns-helper-dnspod")]
struct Args {
    #[structopt(long)]
    domain: String,
    #[structopt(long)]
    proof: String,
    #[structopt(long)]
    clean: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let args = Args::from_args();

    info!("args = {:?}", args);
    Ok(())
}
