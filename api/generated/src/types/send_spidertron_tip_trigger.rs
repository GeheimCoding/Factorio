#[derive(Debug, serde::Deserialize)]
pub struct SendSpidertronTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_append")]
    append: bool,
}
fn default_append() -> bool {
    false
}
