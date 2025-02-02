#[derive(Debug, serde::Deserialize)]
pub struct RocketSiloRocketShadowPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
}
