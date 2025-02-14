#[derive(Debug, serde::Deserialize)]
pub struct RailFencePictureSet {
    ends: (
        Box<crate::types::RailFenceDirectionSet>,
        Box<crate::types::RailFenceDirectionSet>,
        Box<crate::types::RailFenceDirectionSet>,
        Box<crate::types::RailFenceDirectionSet>,
    ),
    ends_upper: Option<(
        Box<crate::types::RailFenceDirectionSet>,
        Box<crate::types::RailFenceDirectionSet>,
        Box<crate::types::RailFenceDirectionSet>,
        Box<crate::types::RailFenceDirectionSet>,
    )>,
    fence: crate::types::RailFenceDirectionSet,
    fence_upper: Option<crate::types::RailFenceDirectionSet>,
}
