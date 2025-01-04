pub struct TileTransitionsVariants {
    empty_transitions: bool,
    light: Vec<TileLightPictures>,
    main: Vec<TileMainPictures>,
    material_background: MaterialTextureParameters,
    material_light: MaterialTextureParameters,
    material_texture_height_in_tiles: u8,
    material_texture_width_in_tiles: u8,
    transition: TileTransitions,
}
