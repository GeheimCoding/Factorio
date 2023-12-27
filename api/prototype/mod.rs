use std::io;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PrototypeApiFormat {}

impl PrototypeApiFormat {
    pub fn generate_prototype_api(&self) -> io::Result<()> {
        Ok(())
    }
}
