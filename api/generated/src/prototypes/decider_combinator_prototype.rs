#[derive(Debug, serde::Deserialize)]
pub struct DeciderCombinatorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::CombinatorPrototype,
    equal_symbol_sprites: Option<crate::types::Sprite4Way>,
    greater_or_equal_symbol_sprites: Option<crate::types::Sprite4Way>,
    greater_symbol_sprites: Option<crate::types::Sprite4Way>,
    less_or_equal_symbol_sprites: Option<crate::types::Sprite4Way>,
    less_symbol_sprites: Option<crate::types::Sprite4Way>,
    not_equal_symbol_sprites: Option<crate::types::Sprite4Way>,
}
