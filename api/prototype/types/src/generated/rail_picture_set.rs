pub struct RailPictureSet {
    back_rail_endings: Sprite16Way,
    east: RailPieceLayers,
    fog_mask: RailsFogMaskDefinitions,
    front_rail_endings: Sprite16Way,
    north: RailPieceLayers,
    northeast: RailPieceLayers,
    northwest: RailPieceLayers,
    rail_endings: Sprite16Way,
    render_layers: RailRenderLayers,
    secondary_render_layers: RailRenderLayers,
    segment_visualisation_endings: RotatedAnimation,
    slice_origin: RailsSliceOffsets,
    south: RailPieceLayers,
    southeast: RailPieceLayers,
    southwest: RailPieceLayers,
    west: RailPieceLayers,
}
