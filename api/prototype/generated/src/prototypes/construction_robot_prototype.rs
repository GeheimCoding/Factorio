pub struct ConstructionRobotPrototype {
    collision_box: crate::types::BoundingBox,
    construction_vector: crate::types::Vector,
    mined_sound_volume_modifier: f32,
    repairing_sound: crate::types::Sound,
    shadow_working: crate::types::RotatedAnimation,
    smoke: crate::types::Animation,
    sparks: crate::types::AnimationVariations,
    working: crate::types::RotatedAnimation,
    working_light: crate::types::LightDefinition,
}
