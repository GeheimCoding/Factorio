#[derive(serde::Deserialize)]
pub struct EntityGhostPrototype {
    base_: crate::prototypes::EntityPrototype,
    huge_build_animated_sound: Option<crate::types::Sound>,
    huge_build_sound: Option<crate::types::Sound>,
    large_build_animated_sound: Option<crate::types::Sound>,
    large_build_sound: Option<crate::types::Sound>,
    medium_build_animated_sound: Option<crate::types::Sound>,
    medium_build_sound: Option<crate::types::Sound>,
    small_build_animated_sound: Option<crate::types::Sound>,
}
