use clap::Parser;
use log::info;

mod defs;
mod dnspod;
mod domain_name;

use defs::*;

#[derive(Parser, Debug)]
#[command(name = "acmed-dns-helper-dnspod", version)]
struct Args {
    #[arg(long)]
    domain: String,
    #[arg(long, allow_hyphen_values = true)]
    proof: String,
    #[arg(long)]
    clean: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    info!("args = {:?}", args);

    let mut backend = dnspod::DNSPodBackend::new()?;

    if args.clean {
        backend.do_clean(&args.domain, &args.proof).await
    } else {
        backend.do_provision(&args.domain, &args.proof).await
    }
}
