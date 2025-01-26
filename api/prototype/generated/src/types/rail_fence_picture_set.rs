#[derive(Debug, serde::Deserialize)]
pub struct RailFencePictureSet {
    ends: (
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
    ),
    ends_upper: Option<(
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
        crate::types::RailFenceDirectionSet,
    )>,
    fence: crate::types::RailFenceDirectionSet,
    fence_upper: Option<crate::types::RailFenceDirectionSet>,
}
