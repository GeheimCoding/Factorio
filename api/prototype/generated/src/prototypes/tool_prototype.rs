#[derive(serde::Deserialize)]
pub struct ToolPrototype {
    base_: crate::prototypes::ItemPrototype,
    durability: Option<f64>,
    durability_description_key: Option<String>,
    durability_description_value: Option<String>,
    #[serde(default = "default_infinite")]
    infinite: bool,
}
fn default_infinite() -> bool {
    false
}
