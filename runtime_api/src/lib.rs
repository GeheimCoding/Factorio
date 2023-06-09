mod generated;
pub use generated::*;

pub fn parse_factorio_type(json: &str) -> Result<FactorioType, serde_json::Error> {
    serde_json::from_str(&json)
}
