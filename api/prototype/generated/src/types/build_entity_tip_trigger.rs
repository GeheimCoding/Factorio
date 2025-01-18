#[derive(serde::Deserialize)]
pub struct BuildEntityTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    build_by_dragging: bool,
    #[serde(default = "default_build_in_line")]
    build_in_line: bool,
    #[serde(default = "default_consecutive")]
    consecutive: bool,
    entity: crate::types::EntityID,
    #[serde(default = "default_linear_power_pole_line")]
    linear_power_pole_line: bool,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    quality: crate::types::QualityID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_build_in_line() -> bool {
    false
}
fn default_consecutive() -> bool {
    false
}
fn default_linear_power_pole_line() -> bool {
    false
}
fn default_match_type_only() -> bool {
    false
}
