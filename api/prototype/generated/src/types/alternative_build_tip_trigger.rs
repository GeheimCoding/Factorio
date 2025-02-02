#[derive(Debug, serde::Deserialize)]
pub struct AlternativeBuildTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
