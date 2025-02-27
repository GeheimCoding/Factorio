#[derive(Debug, serde::Deserialize)]
pub enum StatelessVisualisations {
    #[serde(untagged)]
    StatelessVisualisation(Box<crate::types::StatelessVisualisation>),
    #[serde(untagged)]
    VecStatelessVisualisation(crate::vec::Vec<crate::types::StatelessVisualisation>),
}
