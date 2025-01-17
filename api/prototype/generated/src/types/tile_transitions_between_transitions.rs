#[derive(serde::Deserialize)]
pub struct TileTransitionsBetweenTransitions {
    base_: crate::types::TileTransitions,
    transition_group1: u8,
    transition_group2: u8,
}
