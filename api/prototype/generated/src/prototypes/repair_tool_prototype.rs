#[derive(Debug, serde::Deserialize)]
pub struct RepairToolPrototype {
    base_: crate::prototypes::ToolPrototype,
    speed: f32,
}
