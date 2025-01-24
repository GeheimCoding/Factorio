#[derive(serde::Deserialize)]
pub struct ConstructionRobotPrototype {
    base_: crate::prototypes::RobotWithLogisticInterfacePrototype,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: Option<crate::types::BoundingBox>,
    construction_vector: crate::types::Vector,
    #[serde(default = "default_mined_sound_volume_modifier")]
    mined_sound_volume_modifier: f32,
    repairing_sound: Option<crate::types::Sound>,
    shadow_working: Option<crate::types::RotatedAnimation>,
    smoke: Option<crate::types::Animation>,
    sparks: Option<crate::types::AnimationVariations>,
    working: Option<crate::types::RotatedAnimation>,
    working_light: Option<crate::types::LightDefinition>,
}
fn default_mined_sound_volume_modifier() -> f32 {
    1.0
}
