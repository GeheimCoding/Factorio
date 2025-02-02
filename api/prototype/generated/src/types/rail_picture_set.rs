#[derive(Debug, serde::Deserialize)]
pub struct RailPictureSet {
    // default: Value of `rail_endings`
    back_rail_endings: Option<crate::types::Sprite16Way>,
    east: crate::types::RailPieceLayers,
    fog_mask: Option<RailsFogMaskDefinitions>,
    // default: Value of `rail_endings`
    front_rail_endings: Option<crate::types::Sprite16Way>,
    north: crate::types::RailPieceLayers,
    northeast: crate::types::RailPieceLayers,
    northwest: crate::types::RailPieceLayers,
    rail_endings: Option<crate::types::Sprite16Way>,
    render_layers: crate::types::RailRenderLayers,
    // default: Value of `render_layers`
    secondary_render_layers: Option<crate::types::RailRenderLayers>,
    segment_visualisation_endings: Option<crate::types::RotatedAnimation>,
    slice_origin: Option<RailsSliceOffsets>,
    south: crate::types::RailPieceLayers,
    southeast: crate::types::RailPieceLayers,
    southwest: crate::types::RailPieceLayers,
    west: crate::types::RailPieceLayers,
}
#[derive(Debug, serde::Deserialize)]
pub struct RailsFogMaskDefinitions {
    east: Option<crate::types::FogMaskShapeDefinition>,
    north: Option<crate::types::FogMaskShapeDefinition>,
    south: Option<crate::types::FogMaskShapeDefinition>,
    west: Option<crate::types::FogMaskShapeDefinition>,
}
#[derive(Debug, serde::Deserialize)]
pub struct RailsSliceOffsets {
    east: Option<crate::types::Vector>,
    north: Option<crate::types::Vector>,
    south: Option<crate::types::Vector>,
    west: Option<crate::types::Vector>,
}
