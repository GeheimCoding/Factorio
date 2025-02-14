#[derive(Debug, serde::Deserialize)]
pub struct RailSignalPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RailSignalBasePrototype,
}
