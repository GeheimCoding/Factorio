#[derive(Debug, serde::Deserialize)]
pub struct RailSignalColorToFrameIndex {
    #[serde(default = "default_blue")]
    blue: u8,
    #[serde(default = "default_green")]
    green: u8,
    #[serde(default = "default_none")]
    none: u8,
    #[serde(default = "default_red")]
    red: u8,
    #[serde(default = "default_yellow")]
    yellow: u8,
}
fn default_blue() -> u8 {
    0
}
fn default_green() -> u8 {
    0
}
fn default_none() -> u8 {
    0
}
fn default_red() -> u8 {
    0
}
fn default_yellow() -> u8 {
    0
}
