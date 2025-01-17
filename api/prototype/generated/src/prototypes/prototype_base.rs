#[derive(serde::Deserialize)]
pub struct PrototypeBase {
    factoriopedia_description: crate::types::LocalisedString,
    factoriopedia_simulation: crate::types::SimulationDefinition,
    hidden: bool,
    hidden_in_factoriopedia: bool,
    localised_description: crate::types::LocalisedString,
    localised_name: crate::types::LocalisedString,
    name: String,
    order: crate::types::Order,
    parameter: bool,
    subgroup: crate::types::ItemSubGroupID,
    type_: String,
}
