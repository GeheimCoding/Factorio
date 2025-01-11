pub struct LinkedBeltPrototype {
    allow_blueprint_connection: bool,
    allow_clone_connection: bool,
    allow_side_loading: bool,
    structure: LinkedBeltStructure,
    structure_render_layer: crate::types::RenderLayer,
}
pub struct LinkedBeltStructure {
    back_patch: crate::types::Sprite4Way,
    direction_in: crate::types::Sprite4Way,
    direction_in_side_loading: crate::types::Sprite4Way,
    direction_out: crate::types::Sprite4Way,
    direction_out_side_loading: crate::types::Sprite4Way,
    front_patch: crate::types::Sprite4Way,
}
