#[derive(serde::Deserialize)]
pub struct TrainBrakingForceBonusModifier {
    base_: crate::types::SimpleModifier,
    infer_icon: bool,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
