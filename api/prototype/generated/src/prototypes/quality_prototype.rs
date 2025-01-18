#[derive(serde::Deserialize)]
pub struct QualityPrototype {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_beacon_power_usage_multiplier")]
    beacon_power_usage_multiplier: f32,
    color: crate::types::Color,
    #[serde(default = "default_draw_sprite_by_default")]
    draw_sprite_by_default: bool,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    level: u32,
    #[serde(default = "default_mining_drill_resource_drain_multiplier")]
    mining_drill_resource_drain_multiplier: f32,
    name: String,
    next: crate::types::QualityID,
    #[serde(default = "default_next_probability")]
    next_probability: f64,
    #[serde(default = "default_science_pack_drain_multiplier")]
    science_pack_drain_multiplier: f32,
}
fn default_beacon_power_usage_multiplier() -> f32 {
    1.0
}
fn default_draw_sprite_by_default() -> bool {
    true
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_mining_drill_resource_drain_multiplier() -> f32 {
    1.0
}
fn default_next_probability() -> f64 {
    0.0
}
fn default_science_pack_drain_multiplier() -> f32 {
    1.0
}
