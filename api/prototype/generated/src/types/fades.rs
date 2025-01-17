#[derive(serde::Deserialize)]
pub struct Fades {
    fade_in: crate::types::Fade,
    fade_out: crate::types::Fade,
}
