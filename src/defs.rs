pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait Backend {
    async fn do_clean(&mut self, domain: &str, proof: &str) -> Result<()>;
    async fn do_provision(&mut self, domain: &str, proof: &str) -> Result<()>;
}
