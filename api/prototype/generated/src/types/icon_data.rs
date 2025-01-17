#[derive(serde::Deserialize)]
pub struct IconData {
    draw_background: bool,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    scale: f64,
    shift: crate::types::Vector,
    tint: crate::types::Color,
}
