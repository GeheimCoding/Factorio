#[derive(serde::Deserialize)]
pub struct SendSpidertronTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    append: bool,
    #[serde(rename = "type")]
    type_: String,
}
