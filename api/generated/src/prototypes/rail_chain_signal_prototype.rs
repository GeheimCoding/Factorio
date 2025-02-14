#[derive(Debug, serde::Deserialize)]
pub struct RailChainSignalPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RailSignalBasePrototype,
}
