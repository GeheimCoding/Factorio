#[derive(Debug, serde::Deserialize)]
pub struct TileTransitionsVariants {
    #[serde(default = "default_empty_transitions")]
    empty_transitions: bool,
    light: Option<Vec<crate::types::TileLightPictures>>,
    main: Vec<crate::types::TileMainPictures>,
    material_background: Option<crate::types::MaterialTextureParameters>,
    material_light: Option<crate::types::MaterialTextureParameters>,
    #[serde(default = "default_material_texture_height_in_tiles")]
    material_texture_height_in_tiles: u8,
    #[serde(default = "default_material_texture_width_in_tiles")]
    material_texture_width_in_tiles: u8,
    transition: Option<crate::types::TileTransitions>,
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
