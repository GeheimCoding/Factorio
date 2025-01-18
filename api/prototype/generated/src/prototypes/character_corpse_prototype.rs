#[derive(serde::Deserialize)]
pub struct CharacterCorpsePrototype {
    base_: crate::prototypes::EntityPrototype,
    armor_picture_mapping: std::collections::HashMap<crate::types::ItemID, i32>,
    picture: crate::types::Animation,
    pictures: crate::types::AnimationVariations,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    time_to_live: u32,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
