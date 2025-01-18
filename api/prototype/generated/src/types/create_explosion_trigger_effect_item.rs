#[derive(serde::Deserialize)]
pub struct CreateExplosionTriggerEffectItem {
    base_: crate::types::CreateEntityTriggerEffectItem,
    cycle_while_moving: bool,
    inherit_movement_distance_from_projectile: bool,
    max_movement_distance: f32,
    max_movement_distance_deviation: f32,
    #[serde(rename = "type")]
    type_: String,
}
