#[derive(serde::Deserialize)]
pub struct CyclicSound {
    begin_sound: Option<crate::types::Sound>,
    end_sound: Option<crate::types::Sound>,
    middle_sound: Option<crate::types::Sound>,
}
