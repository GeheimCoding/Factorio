#[derive(Debug, serde::Deserialize)]
pub struct ElevatedHalfDiagonalRailPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::HalfDiagonalRailPrototype,
}
