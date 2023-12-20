use log::{debug, info};

use aliyun_dns::{AliyunDns, DomainRecord};

use crate::defs::{Backend, Result};
use crate::domain_name::get_domain_names_to_use;

pub struct AliyunBackend {
    client: AliyunDns,
}

impl AliyunBackend {
    pub fn new() -> Result<Self> {
        use std::env;

        let akid = env::var("ALIYUN_ACCESSKEY_ID")?;
        let aksecret = env::var("ALIYUN_ACCESSKEY_SECRET")?;

        Ok(Self {
            client: AliyunDns::new(akid, aksecret),
        })
    }
}

async fn list_acme_txt_records(
    client: &AliyunDns,
    domain: &str,
    challenge_record_name: &str,
) -> Result<Vec<DomainRecord>> {
    let records = client.query_domain_records(domain).await?;
    debug!("query_domain_records({:?}) = {:?}", domain, records);
    let result = records
        .domain_records
        .records
        .into_iter()
        .filter(|x| x.record_type == "TXT")
        .filter(|x| x.rr == challenge_record_name)
        .collect();
    Ok(result)
}

impl Backend for AliyunBackend {
    async fn do_clean(&mut self, domain: &str, proof: &str) -> Result<()> {
        let params = get_domain_names_to_use(domain);
        info!("root_domain = {:?}", params.root_domain());
        info!("challenge_record = {:?}", params.challenge_record_name());

        let records = list_acme_txt_records(
            &self.client,
            params.root_domain(),
            params.challenge_record_name(),
        )
        .await?;
        debug!("records = {:?}", records);

        // remove the challenge record
        if records.len() == 0 {
            info!("nothing to clean");
            return Ok(());
        }

        // TODO: remove all records?
        for r in records {
            if r.value != proof {
                debug!("ignoring record with different proof value: {:?}", r);
                continue;
            }

            self.client.delete_domain_record(&r.record_id).await?;
            info!("removed record: {:?}", r);
        }

        Ok(())
    }

    async fn do_provision(&mut self, domain: &str, proof: &str) -> Result<()> {
        todo!()
    }
}
