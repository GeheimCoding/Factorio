#[derive(serde::Deserialize)]
pub struct CircuitNetworkModifier {
    base_: crate::types::BoolModifier,
    type_: String,
    use_icon_overlay_constant: bool,
}
