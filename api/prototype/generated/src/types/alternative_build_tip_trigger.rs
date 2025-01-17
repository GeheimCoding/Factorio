#[derive(serde::Deserialize)]
pub struct AlternativeBuildTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
