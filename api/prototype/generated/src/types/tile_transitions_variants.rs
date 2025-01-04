pub struct TileTransitionsVariants {
    empty_transitions: bool,
    light: Vec<crate::types::TileLightPictures>,
    main: Vec<crate::types::TileMainPictures>,
    material_background: crate::types::MaterialTextureParameters,
    material_light: crate::types::MaterialTextureParameters,
    material_texture_height_in_tiles: u8,
    material_texture_width_in_tiles: u8,
    transition: crate::types::TileTransitions,
}
