#[derive(Debug, serde::Deserialize)]
pub struct PrototypeBase {
    factoriopedia_description: Option<crate::types::LocalisedString>,
    factoriopedia_simulation: Option<crate::types::SimulationDefinition>,
    #[serde(default = "default_hidden")]
    hidden: bool,
    // default: Value of `hidden`
    hidden_in_factoriopedia: Option<bool>,
    localised_description: Option<crate::types::LocalisedString>,
    localised_name: Option<crate::types::LocalisedString>,
    name: Option<String>,
    #[serde(default = "default_order")]
    order: crate::types::Order,
    #[serde(default = "default_parameter")]
    parameter: bool,
    subgroup: Option<crate::types::ItemSubGroupID>,
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
