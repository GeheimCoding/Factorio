#[derive(Debug, serde::Deserialize)]
pub struct FootstepTriggerEffectItem {
    base_: crate::types::CreateParticleTriggerEffectItem,
    actions: Option<Vec<crate::types::CreateParticleTriggerEffectItem>>,
    tiles: Vec<crate::types::TileID>,
    #[serde(default = "default_use_as_default")]
    use_as_default: bool,
}
fn default_use_as_default() -> bool {
    false
}
