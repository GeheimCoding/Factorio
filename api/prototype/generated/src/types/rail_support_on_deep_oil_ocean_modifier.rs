#[derive(serde::Deserialize)]
pub struct RailSupportOnDeepOilOceanModifier {
    base_: crate::types::BoolModifier,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
