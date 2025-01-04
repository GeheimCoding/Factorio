pub struct SurfaceRenderParameters {
    clouds: CloudsEffectProperties,
    day_night_cycle_color_lookup: DaytimeColorLookupTable,
    draw_sprite_clouds: bool,
    fog: FogEffectProperties,
    shadow_opacity: f32,
    space_dust_background: SpaceDustEffectProperties,
    space_dust_foreground: SpaceDustEffectProperties,
    terrain_tint_effect: GlobalTintEffectProperties,
}
