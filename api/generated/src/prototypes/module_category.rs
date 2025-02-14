#[derive(Debug, serde::Deserialize)]
pub struct ModuleCategory {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
