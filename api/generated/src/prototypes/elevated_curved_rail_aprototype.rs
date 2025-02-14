#[derive(Debug, serde::Deserialize)]
pub struct ElevatedCurvedRailAPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::CurvedRailAPrototype,
}
