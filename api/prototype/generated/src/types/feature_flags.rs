#[derive(serde::Deserialize)]
pub struct FeatureFlags {
    expansion_shaders: bool,
    freezing: bool,
    quality: bool,
    rail_bridges: bool,
    segmented_units: bool,
    space_travel: bool,
    spoiling: bool,
}
