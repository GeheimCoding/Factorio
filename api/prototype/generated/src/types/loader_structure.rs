#[derive(serde::Deserialize)]
pub struct LoaderStructure {
    back_patch: crate::types::Sprite4Way,
    direction_in: crate::types::Sprite4Way,
    direction_out: crate::types::Sprite4Way,
    front_patch: crate::types::Sprite4Way,
    frozen_patch_in: crate::types::Sprite4Way,
    frozen_patch_out: crate::types::Sprite4Way,
}
