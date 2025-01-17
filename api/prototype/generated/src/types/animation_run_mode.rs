#[derive(serde::Deserialize)]
pub enum AnimationRunMode {
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "backward")]
    Backward,
    #[serde(rename = "forward_then_backward")]
    ForwardThenBackward,
}
