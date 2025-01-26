#[derive(Debug, serde::Deserialize)]
pub struct SendSpidertronTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_append")]
    append: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_append() -> bool {
    false
}
