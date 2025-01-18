#[derive(serde::Deserialize)]
pub struct LinkedBeltPrototype {
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    #[serde(default = "default_allow_blueprint_connection")]
    allow_blueprint_connection: bool,
    #[serde(default = "default_allow_clone_connection")]
    allow_clone_connection: bool,
    #[serde(default = "default_allow_side_loading")]
    allow_side_loading: bool,
    structure: LinkedBeltStructure,
    #[serde(default = "default_structure_render_layer")]
    structure_render_layer: crate::types::RenderLayer,
}
fn default_allow_blueprint_connection() -> bool {
    true
}
fn default_allow_clone_connection() -> bool {
    true
}
fn default_allow_side_loading() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub struct LinkedBeltStructure {
    back_patch: crate::types::Sprite4Way,
    direction_in: crate::types::Sprite4Way,
    direction_in_side_loading: crate::types::Sprite4Way,
    direction_out: crate::types::Sprite4Way,
    direction_out_side_loading: crate::types::Sprite4Way,
    front_patch: crate::types::Sprite4Way,
}
fn default_structure_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
