#[derive(serde::Deserialize)]
pub struct TransitionApplication {
    offset: bool,
    pod_offset: bool,
    rotation: bool,
}
