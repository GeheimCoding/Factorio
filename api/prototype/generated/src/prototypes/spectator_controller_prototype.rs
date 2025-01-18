#[derive(serde::Deserialize)]
pub struct SpectatorControllerPrototype {
    movement_speed: f64,
    name: String,
    #[serde(rename = "type")]
    type_: String,
}
