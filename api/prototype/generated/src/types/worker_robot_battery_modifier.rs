#[derive(serde::Deserialize)]
pub struct WorkerRobotBatteryModifier {
    base_: crate::types::SimpleModifier,
    infer_icon: bool,
    type_: String,
    use_icon_overlay_constant: bool,
}
