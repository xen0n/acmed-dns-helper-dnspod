use log::debug;
use serde_derive::{Deserialize, Serialize};

use crate::defs::*;

#[derive(Debug)]
pub struct DnspodCredentials {
    id: u64,
    token: String,
}

impl DnspodCredentials {
    pub fn try_from_env() -> Result<Self> {
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
pub struct DnspodUserAgent {
    contact_email: String,
}

impl DnspodUserAgent {
    pub fn try_from_env() -> Result<Self> {
        Ok(Self {
            contact_email: std::env::var("DNSPOD_CONTACT_EMAIL")?,
        })
    }

    fn to_ua_string(&self) -> String {
        // XXX: get version from Cargo.toml
        format!("acmed-dns-helper-dnspod/0.2.0 ({})", self.contact_email)
    }
}

impl From<DnspodUserAgent> for reqwest::header::HeaderValue {
    fn from(a: DnspodUserAgent) -> Self {
        Self::from_str(&a.to_ua_string()).unwrap()
    }
}

#[derive(Debug)]
pub struct DnspodClient<'a> {
    login_token: String,
    client: &'a reqwest::Client,
    api_host: url::Url,
}

const DEFAULT_API_HOST: &str = "https://dnsapi.cn";

impl<'a> DnspodClient<'a> {
    pub fn new(client: &'a reqwest::Client, creds: &DnspodCredentials) -> Self {
        let login_token = creds.to_login_token();

        Self {
            login_token: login_token,
            client: client,
            api_host: DEFAULT_API_HOST.parse().unwrap(),
        }
    }

    pub async fn list_acme_txt_records<S: AsRef<str>, T: AsRef<str>>(
        &self,
        domain: S,
        subdomain: T,
    ) -> Result<Vec<DnspodRespRecord>> {
        #[derive(Serialize)]
        struct Params<'a, 'b, 'c> {
            login_token: &'a str,
            format: &'static str,
            domain: &'b str,
            sub_domain: &'c str,
            record_type: &'static str,
        }

        let params = Params {
            login_token: &self.login_token,
            format: "json",
            domain: domain.as_ref(),
            sub_domain: subdomain.as_ref(),
            record_type: "TXT",
        };

        let resp = self
            .client
            .post(self.api_host.join("Record.List").unwrap())
            .form(&params)
            .send()
            .await?
            .json::<DnspodRespRecordList>()
            .await?;

        match resp.status.try_parse_err() {
            Ok(_) => Ok(resp.records.unwrap()),
            Err(e) => match e.code {
                10 => Ok(vec![]),
                _ => Err(Box::new(e)),
            },
        }
    }

    pub async fn create_acme_challenge_record<S: AsRef<str>, T: AsRef<str>, U: AsRef<str>>(
        &self,
        domain: S,
        subdomain: T,
        proof: U,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Params<'a, 'b, 'c, 'd> {
            login_token: &'a str,
            format: &'static str,
            domain: &'b str,
            sub_domain: &'c str,
            record_type: &'static str,
            record_line: &'static str, // wtf this is mandatory
            value: &'d str,
        }

        let params = Params {
            login_token: &self.login_token,
            format: "json",
            domain: domain.as_ref(),
            sub_domain: subdomain.as_ref(),
            record_type: "TXT",
            record_line: "默认",
            value: proof.as_ref(),
        };

        let resp = self
            .client
            .post(self.api_host.join("Record.Create").unwrap())
            .form(&params)
            .send()
            .await?
            .json::<DnspodRespRecordCreate>()
            .await?;

        match resp.status.try_parse_err() {
            Ok(_) => {
                debug!("record created: {:?}", resp.record.unwrap());
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn remove_domain_record<S: AsRef<str>>(
        &self,
        domain: S,
        record_id: i64,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Params<'a, 'b> {
            login_token: &'a str,
            format: &'static str,
            domain: &'b str,
            record_id: i64,
        }

        let params = Params {
            login_token: &self.login_token,
            format: "json",
            domain: domain.as_ref(),
            record_id: record_id,
        };

        let resp = self
            .client
            .post(self.api_host.join("Record.Remove").unwrap())
            .form(&params)
            .send()
            .await?
            .json::<DnspodRespRecordRemove>()
            .await?;

        match resp.status.try_parse_err() {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[derive(Debug, Deserialize)]
struct DnspodRespStatus {
    code: String,
    message: String,
    created_at: String,
}

#[derive(Debug)]
struct DnspodError {
    code: i64,
    message: String,
}

impl std::fmt::Display for DnspodError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DnspodError({}, \"{}\")", self.code, self.message)
    }
}

// this is rooted directly in the response so no source
impl std::error::Error for DnspodError {}

impl DnspodRespStatus {
    fn try_parse_err(&self) -> std::result::Result<(), DnspodError> {
        // XXX fix this unwrap
        let errcode = self.code.parse().unwrap();
        match errcode {
            1 => Ok(()),
            _ => Err(DnspodError {
                code: errcode,
                message: self.message.clone(),
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
struct DnspodRespRecordCreate {
    status: DnspodRespStatus,
    record: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct DnspodRespRecordRemove {
    status: DnspodRespStatus,
}

#[derive(Debug, Deserialize)]
struct DnspodRespRecordList {
    status: DnspodRespStatus,
    // domain: DnspodRespDomain,
    // info: DnspodRespInfo,
    records: Option<Vec<DnspodRespRecord>>,
}

#[derive(Debug, Deserialize)]
pub struct DnspodRespRecord {
    pub id: String,
    ttl: String,
    pub value: String,
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
