#[derive(Debug, serde::Deserialize)]
pub struct SurfaceRenderParameters {
    clouds: Option<crate::types::CloudsEffectProperties>,
    day_night_cycle_color_lookup: Option<crate::types::DaytimeColorLookupTable>,
    #[serde(default = "default_draw_sprite_clouds")]
    draw_sprite_clouds: bool,
    fog: Option<crate::types::FogEffectProperties>,
    #[serde(default = "default_shadow_opacity")]
    shadow_opacity: f32,
    space_dust_background: Option<crate::types::SpaceDustEffectProperties>,
    space_dust_foreground: Option<crate::types::SpaceDustEffectProperties>,
    terrain_tint_effect: Option<crate::types::GlobalTintEffectProperties>,
}
fn default_draw_sprite_clouds() -> bool {
    false
}
fn default_shadow_opacity() -> f32 {
    0.5
}
