#[derive(serde::Deserialize)]
pub struct PipePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    fluid_box: crate::types::FluidBox,
    horizontal_window_bounding_box: crate::types::BoundingBox,
    pictures: Option<crate::types::PipePictures>,
    vertical_window_bounding_box: crate::types::BoundingBox,
}
