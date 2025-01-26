#[derive(Debug, serde::Deserialize)]
pub struct SurfacePrototype {
    base_: crate::prototypes::Prototype,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    surface_properties: Option<std::collections::HashMap<crate::types::SurfacePropertyID, f64>>,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
