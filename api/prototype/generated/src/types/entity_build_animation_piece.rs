#[derive(Debug, serde::Deserialize)]
pub struct EntityBuildAnimationPiece {
    body: crate::types::Animation,
    top: crate::types::Animation,
}
