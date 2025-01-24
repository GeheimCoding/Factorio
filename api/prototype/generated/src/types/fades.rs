#[derive(serde::Deserialize)]
pub struct Fades {
    fade_in: Option<crate::types::Fade>,
    fade_out: Option<crate::types::Fade>,
}
