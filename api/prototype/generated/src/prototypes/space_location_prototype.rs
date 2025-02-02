#[derive(Debug, serde::Deserialize)]
pub struct SpaceLocationPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    asteroid_spawn_definitions:
        Option<crate::vec::Vec<crate::types::SpaceLocationAsteroidSpawnDefinition>>,
    #[serde(default = "default_asteroid_spawn_influence")]
    asteroid_spawn_influence: f64,
    #[serde(default = "default_auto_save_on_first_trip")]
    auto_save_on_first_trip: bool,
    distance: f64,
    #[serde(default = "default_draw_orbit")]
    draw_orbit: bool,
    #[serde(default = "default_fly_condition")]
    fly_condition: bool,
    gravity_pull: Option<f64>,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<crate::vec::Vec<crate::types::IconData>>,
    #[serde(default = "default_label_orientation")]
    label_orientation: crate::types::RealOrientation,
    #[serde(default = "default_magnitude")]
    magnitude: f64,
    orientation: crate::types::RealOrientation,
    // default: same as orientation
    parked_platforms_orientation: Option<crate::types::RealOrientation>,
    planet_procession_set: Option<crate::types::ProcessionSet>,
    platform_procession_set: Option<crate::types::ProcessionSet>,
    procession_audio_catalogue: Option<crate::types::ProcessionAudioCatalogue>,
    procession_graphic_catalogue: Option<crate::types::ProcessionGraphicCatalogue>,
    #[serde(default = "default_solar_power_in_space")]
    solar_power_in_space: f64,
    starmap_icon: Option<crate::types::FileName>,
    #[serde(default = "default_starmap_icon_size")]
    starmap_icon_size: crate::types::SpriteSizeType,
    starmap_icons: Option<crate::vec::Vec<crate::types::IconData>>,
}
fn default_asteroid_spawn_influence() -> f64 {
    0.1
}
fn default_auto_save_on_first_trip() -> bool {
    true
}
fn default_draw_orbit() -> bool {
    true
}
fn default_fly_condition() -> bool {
    false
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_label_orientation() -> crate::types::RealOrientation {
    0.2
}
fn default_magnitude() -> f64 {
    1.0
}
fn default_solar_power_in_space() -> f64 {
    1.0
}
fn default_starmap_icon_size() -> crate::types::SpriteSizeType {
    64
}
