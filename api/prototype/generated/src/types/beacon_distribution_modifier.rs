#[derive(serde::Deserialize)]
pub struct BeaconDistributionModifier {
    base_: crate::types::SimpleModifier,
    infer_icon: bool,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
