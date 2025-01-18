#[derive(serde::Deserialize)]
pub struct IconData {
    // default: `true` for the first layer, `false` otherwise
    draw_background: bool,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    scale: f64,
    // default: `{0, 0}`
    shift: crate::types::Vector,
    // default: `{r=1, g=1, b=1, a=1}`
    tint: crate::types::Color,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
