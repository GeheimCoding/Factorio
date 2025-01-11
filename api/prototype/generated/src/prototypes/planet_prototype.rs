pub struct PlanetPrototype {
    base_: crate::prototypes::SpaceLocationPrototype,
    entities_require_heating: bool,
    lightning_properties: crate::types::LightningProperties,
    map_gen_settings: crate::types::PlanetPrototypeMapGenSettings,
    map_seed_offset: u32,
    persistent_ambient_sounds: crate::types::PersistentWorldAmbientSoundsDefinition,
    player_effects: crate::types::Trigger,
    pollutant_type: crate::types::AirbornePollutantID,
    surface_properties: std::collections::HashMap<crate::types::SurfacePropertyID, f64>,
    surface_render_parameters: crate::types::SurfaceRenderParameters,
    ticks_between_player_effects: crate::types::MapTick,
}
