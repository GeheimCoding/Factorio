#[derive(serde::Deserialize)]
pub struct TurretGraphicsSet {
    base_visualisation: Option<TurretGraphicsSetBaseVisualisation>,
}
#[derive(serde::Deserialize)]
pub enum TurretGraphicsSetBaseVisualisation {
    #[serde(untagged)]
    TurretBaseVisualisation(Box<crate::types::TurretBaseVisualisation>),
    #[serde(untagged)]
    VecTurretBaseVisualisation(Vec<crate::types::TurretBaseVisualisation>),
}
