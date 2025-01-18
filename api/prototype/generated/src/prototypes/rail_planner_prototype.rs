#[derive(serde::Deserialize)]
pub struct RailPlannerPrototype {
    base_: crate::prototypes::ItemPrototype,
    // default: 8 * 2 + 1.41 + 0.5
    manual_length_limit: f64,
    rails: Vec<crate::types::EntityID>,
    support: crate::types::EntityID,
}
