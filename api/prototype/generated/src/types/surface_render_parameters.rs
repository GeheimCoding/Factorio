#[derive(serde::Deserialize)]
pub struct SurfaceRenderParameters {
    clouds: crate::types::CloudsEffectProperties,
    day_night_cycle_color_lookup: crate::types::DaytimeColorLookupTable,
    draw_sprite_clouds: bool,
    fog: crate::types::FogEffectProperties,
    shadow_opacity: f32,
    space_dust_background: crate::types::SpaceDustEffectProperties,
    space_dust_foreground: crate::types::SpaceDustEffectProperties,
    terrain_tint_effect: crate::types::GlobalTintEffectProperties,
}
