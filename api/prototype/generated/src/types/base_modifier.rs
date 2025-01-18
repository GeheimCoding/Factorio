#[derive(serde::Deserialize)]
pub struct BaseModifier {
    #[serde(default = "default_hidden")]
    hidden: bool,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
}
fn default_hidden() -> bool {
    false
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
