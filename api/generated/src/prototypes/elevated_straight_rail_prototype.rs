#[derive(Debug, serde::Deserialize)]
pub struct ElevatedStraightRailPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::StraightRailPrototype,
}
