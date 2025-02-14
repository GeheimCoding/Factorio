#[derive(Debug, serde::Deserialize)]
pub struct RepairToolPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ToolPrototype,
    speed: f32,
}
