#[derive(serde::Deserialize)]
pub struct TrainStopLight {
    light: crate::types::LightDefinition,
    picture: crate::types::Sprite4Way,
    red_picture: crate::types::Sprite4Way,
}
