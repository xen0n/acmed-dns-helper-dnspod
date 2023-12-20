use clap::{Parser, ValueEnum};
use log::info;

mod aliyun;
mod defs;
mod dnspod;
mod domain_name;

use defs::*;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum BackendType {
    Dnspod,
    Aliyun,
}

#[derive(Parser, Debug)]
#[command(name = "acmed-dns-helper-dnspod", version)]
struct Args {
    #[arg(long, value_enum)]
    backend: Option<BackendType>,
    #[arg(long)]
    domain: String,
    #[arg(long, allow_hyphen_values = true)]
    proof: String,
    #[arg(long)]
    clean: bool,
}

// workaround the fact that trait Backend isn't object-safe
enum BoundBackend {
    Dnspod(dnspod::DNSPodBackend),
    Aliyun(aliyun::AliyunBackend),
}

impl Backend for BoundBackend {
    async fn do_clean(&mut self, domain: &str, proof: &str) -> Result<()> {
        match self {
            Self::Dnspod(x) => x.do_clean(domain, proof).await,
            Self::Aliyun(x) => x.do_clean(domain, proof).await,
        }
    }

    async fn do_provision(&mut self, domain: &str, proof: &str) -> Result<()> {
        match self {
            Self::Dnspod(x) => x.do_provision(domain, proof).await,
            Self::Aliyun(x) => x.do_provision(domain, proof).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    info!("args = {:?}", args);

    let backend_type = args.backend.unwrap_or(BackendType::Dnspod);
    let mut backend = match backend_type {
        BackendType::Dnspod => BoundBackend::Dnspod(dnspod::DNSPodBackend::new()?),
        BackendType::Aliyun => BoundBackend::Aliyun(aliyun::AliyunBackend::new()?),
    };

    if args.clean {
        backend.do_clean(&args.domain, &args.proof).await
    } else {
        backend.do_provision(&args.domain, &args.proof).await
    }
}
