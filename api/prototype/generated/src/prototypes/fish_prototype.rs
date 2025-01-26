#[derive(Debug, serde::Deserialize)]
pub struct FishPrototype {
    base_: crate::prototypes::EntityWithHealthPrototype,
    pictures: Option<crate::types::SpriteVariations>,
}
