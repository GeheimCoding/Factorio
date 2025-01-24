#[derive(serde::Deserialize)]
pub struct PlanetPrototype {
    base_: crate::prototypes::SpaceLocationPrototype,
    #[serde(default = "default_entities_require_heating")]
    entities_require_heating: bool,
    lightning_properties: Option<crate::types::LightningProperties>,
    map_gen_settings: Option<crate::types::PlanetPrototypeMapGenSettings>,
    map_seed_offset: Option<u32>,
    persistent_ambient_sounds: Option<crate::types::PersistentWorldAmbientSoundsDefinition>,
    player_effects: Option<crate::types::Trigger>,
    pollutant_type: Option<crate::types::AirbornePollutantID>,
    surface_properties: Option<std::collections::HashMap<crate::types::SurfacePropertyID, f64>>,
    surface_render_parameters: Option<crate::types::SurfaceRenderParameters>,
    #[serde(default = "default_ticks_between_player_effects")]
    ticks_between_player_effects: crate::types::MapTick,
}
fn default_entities_require_heating() -> bool {
    false
}
fn default_ticks_between_player_effects() -> crate::types::MapTick {
    0
}
