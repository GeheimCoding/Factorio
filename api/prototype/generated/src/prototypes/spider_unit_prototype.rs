pub struct SpiderUnitPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    absorptions_to_join_attack: std::collections::HashMap<crate::types::AirbornePollutantID, f32>,
    ai_settings: crate::types::UnitAISettings,
    attack_parameters: crate::types::AttackParameters,
    distraction_cooldown: u32,
    dying_sound: crate::types::Sound,
    graphics_set: crate::types::SpiderTorsoGraphicsSet,
    height: f32,
    max_pursue_distance: f64,
    min_pursue_time: u32,
    radar_range: u32,
    spawning_time_modifier: f64,
    spider_engine: crate::types::SpiderEngineSpecification,
    torso_bob_speed: f32,
    torso_rotation_speed: f32,
    vision_distance: f64,
    warcry: crate::types::Sound,
}
