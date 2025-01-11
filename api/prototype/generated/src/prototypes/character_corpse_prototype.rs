pub struct CharacterCorpsePrototype {
    armor_picture_mapping: std::collections::HashMap<crate::types::ItemID, i32>,
    picture: crate::types::Animation,
    pictures: crate::types::AnimationVariations,
    render_layer: crate::types::RenderLayer,
    time_to_live: u32,
}
