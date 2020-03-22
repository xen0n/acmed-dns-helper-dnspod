use log::info;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
    fn try_from_env() -> Result<Self> {
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

#[derive(Debug)]
struct DnspodUserAgent {
    contact_email: String,
}

impl DnspodUserAgent {
    fn try_from_env() -> Result<Self> {
        Ok(Self {
            contact_email: std::env::var("DNSPOD_CONTACT_EMAIL")?,
        })
    }

    fn to_ua_string(&self) -> String {
        // XXX: get version from Cargo.toml
        format!("acmed-dns-helper-dnspod/0.1.0 ({})", self.contact_email)
    }
}

impl From<DnspodUserAgent> for reqwest::header::HeaderValue {
    fn from(a: DnspodUserAgent) -> Self {
        Self::from_str(&a.to_ua_string()).unwrap()
    }
}

#[derive(Debug)]
struct DnspodClient<'a> {
    login_token: String,
    client: &'a reqwest::Client,
    api_host: url::Url,
}

const DEFAULT_API_HOST: &str = "https://dnsapi.cn";

impl<'a> DnspodClient<'a> {
    fn new(client: &'a reqwest::Client, creds: &DnspodCredentials) -> Self {
        let login_token = creds.to_login_token();

        Self {
            login_token: login_token,
            client: client,
            api_host: DEFAULT_API_HOST.parse().unwrap(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::from_args();
    let dnspod_creds = DnspodCredentials::try_from_env()?;
    let dnspod_ua = DnspodUserAgent::try_from_env()?;

    info!("args = {:?}", args);
    info!("dnspod creds = {:?}", dnspod_creds);
    info!("dnspod_ua = {:?}", dnspod_ua);

    let http = reqwest::ClientBuilder::new()
        .user_agent(dnspod_ua)
        .build()?;

    let _dnspod = DnspodClient::new(&http, &dnspod_creds);

    Ok(())
}
