use clap::Parser;
use log::{debug, info};

mod defs;
mod dnspod;
mod domain_name;

use defs::*;
use domain_name::get_domain_names_to_use;

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
    let dnspod_creds = dnspod::DnspodCredentials::try_from_env()?;
    let dnspod_ua = dnspod::DnspodUserAgent::try_from_env()?;

    info!("args = {:?}", args);
    info!("dnspod creds = {:?}", dnspod_creds);
    info!("dnspod_ua = {:?}", dnspod_ua);

    let (root_domain, challenge_record) = get_domain_names_to_use(&args.domain);
    info!("root_domain = {:?}", root_domain);
    info!("challenge_record = {:?}", &challenge_record);

    let http = reqwest::ClientBuilder::new()
        .user_agent(dnspod_ua)
        .build()?;

    let dnspod = dnspod::DnspodClient::new(&http, &dnspod_creds);

    let records = dnspod
        .list_acme_txt_records(root_domain, &challenge_record)
        .await?;
    debug!("records = {:?}", records);

    if args.clean {
        // remove the challenge record
        if records.len() == 0 {
            info!("nothing to clean");
            return Ok(());
        }

        // TODO: remove all records?
        for r in records {
            if r.value != args.proof {
                debug!("ignoring record with different proof value: {:?}", r);
                continue;
            }

            dnspod
                .remove_domain_record(root_domain, r.id.parse()?)
                .await?;
            info!("removed record: {:?}", r);
        }
    } else {
        if records.len() > 0 {
            // TODO: doesn't handle enabled status or multiple records yet
            for r in records {
                if r.value == args.proof {
                    info!("a matching record is already present");
                    return Ok(());
                }
            }

            // no matching record
            todo!();
        }

        // add one record
        dnspod
            .create_acme_challenge_record(root_domain, &challenge_record, &args.proof)
            .await?;

        // sleep for a while, because dnspod modifications tend to take a while
        // to be noticed by letsencrypt server which apparently is a bit distanced
        // from China
        info!("dnspod operation successful, wait a bit before return");
        ::futures_timer::Delay::new(std::time::Duration::from_secs(5)).await;
        info!("okay, hope things are set!");
    }

    Ok(())
}
