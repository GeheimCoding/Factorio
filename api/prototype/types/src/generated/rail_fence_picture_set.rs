pub struct RailFencePictureSet {
    ends: (
        RailFenceDirectionSet,
        RailFenceDirectionSet,
        RailFenceDirectionSet,
        RailFenceDirectionSet,
    ),
    ends_upper: (
        RailFenceDirectionSet,
        RailFenceDirectionSet,
        RailFenceDirectionSet,
        RailFenceDirectionSet,
    ),
    fence: RailFenceDirectionSet,
    fence_upper: RailFenceDirectionSet,
}
