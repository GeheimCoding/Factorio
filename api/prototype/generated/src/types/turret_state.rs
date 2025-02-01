#[derive(Debug, serde::Deserialize)]
pub enum TurretState {
    #[serde(rename = "folded")]
    Folded,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "prepared")]
    Prepared,
    #[serde(rename = "starting-attack")]
    StartingAttack,
    #[serde(rename = "attacking")]
    Attacking,
    #[serde(rename = "ending-attack")]
    EndingAttack,
    #[serde(rename = "rotate-for-folding")]
    RotateForFolding,
    #[serde(rename = "folding")]
    Folding,
}
