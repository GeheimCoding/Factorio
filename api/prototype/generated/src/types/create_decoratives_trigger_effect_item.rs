#[derive(serde::Deserialize)]
pub struct CreateDecorativesTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    apply_projection: bool,
    decorative: crate::types::DecorativeID,
    radius_curve: f32,
    spawn_max: u16,
    spawn_max_radius: f32,
    spawn_min: u16,
    spawn_min_radius: f32,
    spread_evenly: bool,
    #[serde(rename = "type")]
    type_: String,
}
