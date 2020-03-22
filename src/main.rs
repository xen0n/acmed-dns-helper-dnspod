use log::{debug, info};
use structopt::StructOpt;
use serde_derive::{Deserialize, Serialize};

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
const ACME_CHALLENGE_SUBDOMAIN: &str = "_acme-challenge";

impl<'a> DnspodClient<'a> {
    fn new(client: &'a reqwest::Client, creds: &DnspodCredentials) -> Self {
        let login_token = creds.to_login_token();

        Self {
            login_token: login_token,
            client: client,
            api_host: DEFAULT_API_HOST.parse().unwrap(),
        }
    }

    async fn list_acme_txt_records<'s, S: AsRef<str>>(&'s self, domain: S) -> Result<DnspodRespRecordList> {
        #[derive(Serialize)]
        struct Params<'a, 'b> {
            login_token: &'a str,
            format: &'static str,
            domain: &'b str,
            sub_domain: &'static str,
            record_type: &'static str,
        }

        let params = Params {
            login_token: &self.login_token,
            format: "json",
            domain: domain.as_ref(),
            sub_domain: ACME_CHALLENGE_SUBDOMAIN,
            record_type: "TXT",
        };

        let resp = self.client
            .post(self.api_host.join("Record.List").unwrap())
            .form(&params)
            .send()
            .await?
            .json::<DnspodRespRecordList>()
            .await?;

        Ok(resp)
    }
}

#[derive(Debug, Deserialize)]
struct DnspodRespStatus {
    code: String,
    message: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct DnspodRespRecordList {
    status: DnspodRespStatus,
    // domain: DnspodRespDomain,
    // info: DnspodRespInfo,
    records: Option<Vec<DnspodRespRecord>>,
}

#[derive(Debug, Deserialize)]
struct DnspodRespRecord {
    id: String,
    ttl: String,
    value: String,
    enabled: String,
    status: String,
    updated_on: String,
    name: String,
    line: String,
    line_id: String,
    #[serde(rename = "type")]
    typ: String,
    weight: Option<String>,
    monitor_status: String,
    remark: String,
    use_aqb: String,
    mx: String,
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

    let dnspod = DnspodClient::new(&http, &dnspod_creds);

    let records = dnspod.list_acme_txt_records(&args.domain).await?;
    debug!("records = {:?}", records);

    Ok(())
}
