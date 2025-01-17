#[derive(serde::Deserialize)]
pub enum StatelessVisualisations {
    #[serde(untagged)]
    StatelessVisualisation(Box<crate::types::StatelessVisualisation>),
    #[serde(untagged)]
    VecStatelessVisualisation(Vec<crate::types::StatelessVisualisation>),
}
