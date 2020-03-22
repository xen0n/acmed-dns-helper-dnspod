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

#[derive(Debug)]
struct DnspodCredentials {
    id: u64,
    token: String,
}

impl DnspodCredentials {
    fn try_from_env() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        use std::env;

        Ok(Self {
            id: env::var("DNSPOD_ID")?.parse()?,
            token: env::var("DNSPOD_TOKEN")?,
        })
    }

    fn to_login_token(&self) -> String {
        format!("{},{}", self.id, self.token)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    let args = Args::from_args();
    let dnspod_creds = DnspodCredentials::try_from_env()?;

    info!("args = {:?}", args);
    info!("dnspod creds = {:?}", dnspod_creds);

    Ok(())
}
