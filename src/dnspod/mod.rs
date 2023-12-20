mod api;

use log::{debug, info};

use crate::defs::{Backend, Result};
use crate::domain_name::get_domain_names_to_use;

pub struct DNSPodBackend {
    client: api::DnspodClient,
}

impl DNSPodBackend {
    pub fn new() -> Result<Self> {
        let dnspod_creds = api::DnspodCredentials::try_from_env()?;
        let dnspod_ua = api::DnspodUserAgent::try_from_env()?;

        info!("dnspod creds = {:?}", dnspod_creds);
        info!("dnspod_ua = {:?}", dnspod_ua);

        let http = reqwest::ClientBuilder::new()
            .user_agent(dnspod_ua)
            .build()?;

        Ok(Self {
            client: api::DnspodClient::new(http, &dnspod_creds),
        })
    }
}

impl Backend for DNSPodBackend {
    async fn do_clean(&mut self, domain: &str, proof: &str) -> crate::defs::Result<()> {
        let params = get_domain_names_to_use(domain);
        info!("root_domain = {:?}", params.root_domain());
        info!("challenge_record = {:?}", params.challenge_record_name());

        let records = self
            .client
            .list_acme_txt_records(params.root_domain(), params.challenge_record_name())
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

            self.client
                .remove_domain_record(params.root_domain(), r.id.parse()?)
                .await?;
            info!("removed record: {:?}", r);
        }

        Ok(())
    }

    async fn do_provision(&mut self, domain: &str, proof: &str) -> crate::defs::Result<()> {
        let params = get_domain_names_to_use(domain);
        info!("root_domain = {:?}", params.root_domain());
        info!("challenge_record = {:?}", params.challenge_record_name());

        let records = self
            .client
            .list_acme_txt_records(params.root_domain(), params.challenge_record_name())
            .await?;
        debug!("records = {:?}", records);

        if records.len() > 0 {
            // TODO: doesn't handle enabled status or multiple records yet
            for r in records {
                if r.value == proof {
                    info!("a matching record is already present");
                    return Ok(());
                }
            }

            // no matching record
            todo!();
        }

        // add one record
        self.client
            .create_acme_challenge_record(
                params.root_domain(),
                params.challenge_record_name(),
                proof,
            )
            .await?;

        // sleep for a while, because dnspod modifications tend to take a while
        // to be noticed by letsencrypt server which apparently is a bit distanced
        // from China
        info!("dnspod operation successful, wait a bit before return");
        ::futures_timer::Delay::new(std::time::Duration::from_secs(5)).await;
        info!("okay, hope things are set!");

        Ok(())
    }
}
