use anyhow::anyhow;
#[allow(unused_imports)]
pub use defines::*;
pub use prototypes::*;
pub use types::*;

pub enum FactorioType {
    Prototype(Prototypes),
    Type(Types),
}

impl FactorioType {
    pub fn from(input: &str) -> anyhow::Result<FactorioType> {
        let value: serde_json::Value = serde_json::from_str(input)?;
        let serde_type = &value["serde_type"];
        if let serde_json::Value::String(s) = serde_type {
            match s.as_str() {
                "Prototype" => Ok(FactorioType::Prototype(Prototypes::parse(input)?)),
                "Type" => Ok(FactorioType::Type(Types::parse(input)?)),
                _ => Err(anyhow!("unexpected serde_type: {s}")),
            }
        } else {
            Err(anyhow!("invalid serde_type: {serde_type}"))
        }
    }

    pub fn load_data_dump() -> anyhow::Result<Data> {
        todo!()
    }
}
