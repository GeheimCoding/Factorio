#[derive(serde::Deserialize)]
pub struct TileTransitionsVariants {
    #[serde(default = "default_empty_transitions")]
    empty_transitions: bool,
    light: Vec<crate::types::TileLightPictures>,
    main: Vec<crate::types::TileMainPictures>,
    material_background: crate::types::MaterialTextureParameters,
    material_light: crate::types::MaterialTextureParameters,
    #[serde(default = "default_material_texture_height_in_tiles")]
    material_texture_height_in_tiles: u8,
    #[serde(default = "default_material_texture_width_in_tiles")]
    material_texture_width_in_tiles: u8,
    transition: crate::types::TileTransitions,
}
fn default_empty_transitions() -> bool {
    false
}
fn default_material_texture_height_in_tiles() -> u8 {
    8
}
fn default_material_texture_width_in_tiles() -> u8 {
    8
}
