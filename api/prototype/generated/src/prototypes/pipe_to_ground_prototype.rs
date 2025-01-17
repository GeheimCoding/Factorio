#[derive(serde::Deserialize)]
pub struct PipeToGroundPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    disabled_visualization: crate::types::Sprite4Way,
    draw_fluid_icon_override: bool,
    fluid_box: crate::types::FluidBox,
    frozen_patch: crate::types::Sprite4Way,
    pictures: crate::types::Sprite4Way,
    visualization: crate::types::Sprite4Way,
}
