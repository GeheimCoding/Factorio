#[derive(serde::Deserialize)]
pub struct PumpConnectorGraphicsAnimation {
    connector: Option<crate::types::Animation>,
    connector_shadow: Option<crate::types::Animation>,
    standup_base: Option<crate::types::Animation>,
    standup_shadow: Option<crate::types::Animation>,
    standup_top: Option<crate::types::Animation>,
}
