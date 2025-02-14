#[derive(Debug, serde::Deserialize)]
pub struct DamageType {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
