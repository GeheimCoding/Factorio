#[derive(Debug, serde::Deserialize)]
pub struct SpriteParameters {
    base_: crate::types::SpriteSource,
    #[serde(default = "default_apply_runtime_tint")]
    apply_runtime_tint: bool,
    #[serde(default = "default_apply_special_effect")]
    apply_special_effect: bool,
    #[serde(default = "default_blend_mode")]
    blend_mode: crate::types::BlendMode,
    #[serde(default = "default_draw_as_glow")]
    draw_as_glow: bool,
    #[serde(default = "default_draw_as_light")]
    draw_as_light: bool,
    #[serde(default = "default_draw_as_shadow")]
    draw_as_shadow: bool,
    flags: Option<crate::types::SpriteFlags>,
    #[serde(default = "default_generate_sdf")]
    generate_sdf: bool,
    #[serde(default = "default_invert_colors")]
    invert_colors: bool,
    #[serde(default = "default_mipmap_count")]
    mipmap_count: u8,
    #[serde(default = "default_priority")]
    priority: crate::types::SpritePriority,
    #[serde(default = "default_rotate_shift")]
    rotate_shift: bool,
    #[serde(default = "default_scale")]
    scale: f64,
    // default: `{0, 0}`
    shift: Option<crate::types::Vector>,
    #[serde(default = "default_surface")]
    surface: crate::types::SpriteUsageSurfaceHint,
    // default: `{r=1, g=1, b=1, a=1}`
    tint: Option<crate::types::Color>,
    #[serde(default = "default_tint_as_overlay")]
    tint_as_overlay: bool,
    #[serde(default = "default_usage")]
    usage: crate::types::SpriteUsageHint,
}
fn default_apply_runtime_tint() -> bool {
    false
}
fn default_apply_special_effect() -> bool {
    false
}
fn default_blend_mode() -> crate::types::BlendMode {
    crate::types::BlendMode::Normal
}
fn default_draw_as_glow() -> bool {
    false
}
fn default_draw_as_light() -> bool {
    false
}
fn default_draw_as_shadow() -> bool {
    false
}
fn default_generate_sdf() -> bool {
    false
}
fn default_invert_colors() -> bool {
    false
}
fn default_mipmap_count() -> u8 {
    0
}
fn default_priority() -> crate::types::SpritePriority {
    crate::types::SpritePriority::Medium
}
fn default_rotate_shift() -> bool {
    false
}
fn default_scale() -> f64 {
    1.0
}
fn default_surface() -> crate::types::SpriteUsageSurfaceHint {
    crate::types::SpriteUsageSurfaceHint::Any
}
fn default_tint_as_overlay() -> bool {
    false
}
fn default_usage() -> crate::types::SpriteUsageHint {
    crate::types::SpriteUsageHint::Any
}
