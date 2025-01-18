#[derive(serde::Deserialize)]
pub struct RemoteControllerPrototype {
    movement_speed: f64,
    name: String,
    #[serde(rename = "type")]
    type_: String,
}
