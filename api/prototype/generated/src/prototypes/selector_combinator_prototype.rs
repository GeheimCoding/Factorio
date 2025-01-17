#[derive(serde::Deserialize)]
pub struct SelectorCombinatorPrototype {
    base_: crate::prototypes::CombinatorPrototype,
    count_symbol_sprites: crate::types::Sprite4Way,
    max_symbol_sprites: crate::types::Sprite4Way,
    min_symbol_sprites: crate::types::Sprite4Way,
    quality_symbol_sprites: crate::types::Sprite4Way,
    random_symbol_sprites: crate::types::Sprite4Way,
    rocket_capacity_sprites: crate::types::Sprite4Way,
    stack_size_sprites: crate::types::Sprite4Way,
}
