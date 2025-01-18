#[derive(serde::Deserialize)]
pub struct PrototypeBase {
    factoriopedia_description: crate::types::LocalisedString,
    factoriopedia_simulation: crate::types::SimulationDefinition,
    #[serde(default = "default_hidden")]
    hidden: bool,
    // default: Value of `hidden`
    hidden_in_factoriopedia: bool,
    localised_description: crate::types::LocalisedString,
    localised_name: crate::types::LocalisedString,
    name: String,
    #[serde(default = "default_order")]
    order: crate::types::Order,
    #[serde(default = "default_parameter")]
    parameter: bool,
    subgroup: crate::types::ItemSubGroupID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_hidden() -> bool {
    false
}
fn default_order() -> crate::types::Order {
    String::from("")
}
fn default_parameter() -> bool {
    false
}
