#[derive(Debug, serde::Deserialize)]
pub struct LoaderStructure {
    back_patch: Option<crate::types::Sprite4Way>,
    direction_in: Option<crate::types::Sprite4Way>,
    direction_out: Option<crate::types::Sprite4Way>,
    front_patch: Option<crate::types::Sprite4Way>,
    frozen_patch_in: Option<crate::types::Sprite4Way>,
    frozen_patch_out: Option<crate::types::Sprite4Way>,
}
