#[derive(Debug, serde::Deserialize)]
pub struct SelectorCombinatorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::CombinatorPrototype,
    count_symbol_sprites: Option<crate::types::Sprite4Way>,
    max_symbol_sprites: Option<crate::types::Sprite4Way>,
    min_symbol_sprites: Option<crate::types::Sprite4Way>,
    quality_symbol_sprites: Option<crate::types::Sprite4Way>,
    random_symbol_sprites: Option<crate::types::Sprite4Way>,
    rocket_capacity_sprites: Option<crate::types::Sprite4Way>,
    stack_size_sprites: Option<crate::types::Sprite4Way>,
}
