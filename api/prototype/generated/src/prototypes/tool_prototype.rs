#[derive(serde::Deserialize)]
pub struct ToolPrototype {
    base_: crate::prototypes::ItemPrototype,
    durability: f64,
    durability_description_key: String,
    durability_description_value: String,
    infinite: bool,
}
