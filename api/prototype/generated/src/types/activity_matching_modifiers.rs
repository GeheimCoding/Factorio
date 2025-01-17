#[derive(serde::Deserialize)]
pub struct ActivityMatchingModifiers {
    inverted: bool,
    maximum: f32,
    minimum: f32,
    multiplier: f32,
    offset: f32,
}
