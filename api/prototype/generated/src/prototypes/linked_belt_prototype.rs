#[derive(Debug, serde::Deserialize)]
pub struct LinkedBeltPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    #[serde(default = "default_allow_blueprint_connection")]
    allow_blueprint_connection: bool,
    #[serde(default = "default_allow_clone_connection")]
    allow_clone_connection: bool,
    #[serde(default = "default_allow_side_loading")]
    allow_side_loading: bool,
    structure: Option<LinkedBeltStructure>,
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
#[derive(Debug, serde::Deserialize)]
pub struct LinkedBeltStructure {
    back_patch: Option<crate::types::Sprite4Way>,
    direction_in: Option<crate::types::Sprite4Way>,
    direction_in_side_loading: Option<crate::types::Sprite4Way>,
    direction_out: Option<crate::types::Sprite4Way>,
    direction_out_side_loading: Option<crate::types::Sprite4Way>,
    front_patch: Option<crate::types::Sprite4Way>,
}
fn default_structure_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
