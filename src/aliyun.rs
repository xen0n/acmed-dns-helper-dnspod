use crate::defs::{Backend, Result};

pub struct AliyunBackend {}

impl AliyunBackend {
    pub fn new() -> Result<Self> {
        todo!()
    }
}

impl Backend for AliyunBackend {
    async fn do_clean(&mut self, domain: &str, proof: &str) -> Result<()> {
        todo!()
    }

    async fn do_provision(&mut self, domain: &str, proof: &str) -> Result<()> {
        todo!()
    }
}
