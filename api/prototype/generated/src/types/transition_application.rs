#[derive(serde::Deserialize)]
pub struct TransitionApplication {
    #[serde(default = "default_offset")]
    offset: bool,
    #[serde(default = "default_pod_offset")]
    pod_offset: bool,
    #[serde(default = "default_rotation")]
    rotation: bool,
}
fn default_offset() -> bool {
    false
}
fn default_pod_offset() -> bool {
    false
}
fn default_rotation() -> bool {
    false
}
