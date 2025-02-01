#[derive(Debug, serde::Deserialize)]
pub enum AnimationRunMode {
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "backward")]
    Backward,
    #[serde(rename = "forward-then-backward")]
    ForwardThenBackward,
}
