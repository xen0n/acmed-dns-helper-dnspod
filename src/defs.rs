pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const ACME_CHALLENGE_SUBDOMAIN: &str = "_acme-challenge";
