#[derive(serde::Deserialize)]
pub struct ArithmeticCombinatorPrototype {
    base_: crate::prototypes::CombinatorPrototype,
    and_symbol_sprites: Option<crate::types::Sprite4Way>,
    divide_symbol_sprites: Option<crate::types::Sprite4Way>,
    left_shift_symbol_sprites: Option<crate::types::Sprite4Way>,
    minus_symbol_sprites: Option<crate::types::Sprite4Way>,
    modulo_symbol_sprites: Option<crate::types::Sprite4Way>,
    multiply_symbol_sprites: Option<crate::types::Sprite4Way>,
    or_symbol_sprites: Option<crate::types::Sprite4Way>,
    plus_symbol_sprites: Option<crate::types::Sprite4Way>,
    power_symbol_sprites: Option<crate::types::Sprite4Way>,
    right_shift_symbol_sprites: Option<crate::types::Sprite4Way>,
    xor_symbol_sprites: Option<crate::types::Sprite4Way>,
}
