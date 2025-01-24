#[derive(serde::Deserialize)]
pub struct RailRenderLayers {
    // default: Value of `screw`
    back_end: Option<crate::types::RenderLayer>,
    // default: Value of `screw`
    front_end: Option<crate::types::RenderLayer>,
    #[serde(default = "default_metal")]
    metal: crate::types::RenderLayer,
    #[serde(default = "default_screw")]
    screw: crate::types::RenderLayer,
    #[serde(default = "default_stone_path")]
    stone_path: crate::types::RenderLayer,
    #[serde(default = "default_stone_path_lower")]
    stone_path_lower: crate::types::RenderLayer,
    #[serde(default = "default_tie")]
    tie: crate::types::RenderLayer,
    #[serde(default = "default_underwater_layer_offset")]
    underwater_layer_offset: i8,
}
fn default_metal() -> crate::types::RenderLayer {
    crate::types::RenderLayer::RailMetal
}
fn default_screw() -> crate::types::RenderLayer {
    crate::types::RenderLayer::RailScrew
}
fn default_stone_path() -> crate::types::RenderLayer {
    crate::types::RenderLayer::RailStonePath
}
fn default_stone_path_lower() -> crate::types::RenderLayer {
    crate::types::RenderLayer::RailStonePathLower
}
fn default_tie() -> crate::types::RenderLayer {
    crate::types::RenderLayer::RailTie
}
fn default_underwater_layer_offset() -> i8 {
    1
}
