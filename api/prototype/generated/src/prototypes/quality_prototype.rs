#[derive(serde::Deserialize)]
pub struct QualityPrototype {
    base_: crate::prototypes::Prototype,
    beacon_power_usage_multiplier: f32,
    color: crate::types::Color,
    draw_sprite_by_default: bool,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    level: u32,
    mining_drill_resource_drain_multiplier: f32,
    name: String,
    next: crate::types::QualityID,
    next_probability: f64,
    science_pack_drain_multiplier: f32,
}
