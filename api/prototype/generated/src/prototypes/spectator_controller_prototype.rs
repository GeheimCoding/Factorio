#[derive(serde::Deserialize)]
pub struct SpectatorControllerPrototype {
    movement_speed: f64,
    name: String,
    type_: String,
}
