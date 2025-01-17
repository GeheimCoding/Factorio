#[derive(serde::Deserialize)]
pub struct RailPlannerPrototype {
    base_: crate::prototypes::ItemPrototype,
    manual_length_limit: f64,
    rails: Vec<crate::types::EntityID>,
    support: crate::types::EntityID,
}
