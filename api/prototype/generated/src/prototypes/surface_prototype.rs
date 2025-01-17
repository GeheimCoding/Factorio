#[derive(serde::Deserialize)]
pub struct SurfacePrototype {
    base_: crate::prototypes::Prototype,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    surface_properties: std::collections::HashMap<crate::types::SurfacePropertyID, f64>,
}
