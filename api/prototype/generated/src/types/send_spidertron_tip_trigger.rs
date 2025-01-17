#[derive(serde::Deserialize)]
pub struct SendSpidertronTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    append: bool,
    type_: String,
}
