#[derive(serde::Deserialize)]
pub struct BaseModifier {
    hidden: bool,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
}
