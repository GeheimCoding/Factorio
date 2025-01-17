#[derive(serde::Deserialize)]
pub struct RailFencePictureSet {
    ends: (
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
    ),
    ends_upper: (
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
    ),
    fence: crate::types::RailFenceDirectionSet,
    fence_upper: crate::types::RailFenceDirectionSet,
}
