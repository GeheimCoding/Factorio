#[derive(serde::Deserialize)]
pub struct ConstructionRobotPrototype {
    base_: crate::prototypes::RobotWithLogisticInterfacePrototype,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: crate::types::BoundingBox,
    construction_vector: crate::types::Vector,
    #[serde(default = "default_mined_sound_volume_modifier")]
    mined_sound_volume_modifier: f32,
    repairing_sound: crate::types::Sound,
    shadow_working: crate::types::RotatedAnimation,
    smoke: crate::types::Animation,
    sparks: crate::types::AnimationVariations,
    working: crate::types::RotatedAnimation,
    working_light: crate::types::LightDefinition,
}
fn default_mined_sound_volume_modifier() -> f32 {
    1.0
}
