#[derive(Debug, serde::Deserialize)]
pub struct ElevatedCurvedRailBPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::CurvedRailBPrototype,
}
