#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum BuildCheckType {
    BlueprintGhost = 4,
    GhostRevive = 5,
    Manual = 1,
    ManualGhost = 3,
    Script = 0,
    ScriptGhost = 2,
}
