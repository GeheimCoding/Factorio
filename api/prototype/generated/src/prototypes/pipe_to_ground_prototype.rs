#[derive(Debug, serde::Deserialize)]
pub struct PipeToGroundPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    disabled_visualization: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_draw_fluid_icon_override")]
    draw_fluid_icon_override: bool,
    fluid_box: crate::types::FluidBox,
    frozen_patch: Option<crate::types::Sprite4Way>,
    pictures: Option<crate::types::Sprite4Way>,
    visualization: Option<crate::types::Sprite4Way>,
}
fn default_draw_fluid_icon_override() -> bool {
    false
}
