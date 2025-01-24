#[derive(serde::Deserialize)]
pub struct CapsulePrototype {
    base_: crate::prototypes::ItemPrototype,
    capsule_action: crate::types::CapsuleAction,
    radius_color: Option<crate::types::Color>,
}
