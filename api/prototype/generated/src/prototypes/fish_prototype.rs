#[derive(serde::Deserialize)]
pub struct FishPrototype {
    base_: crate::prototypes::EntityWithHealthPrototype,
    pictures: crate::types::SpriteVariations,
}
