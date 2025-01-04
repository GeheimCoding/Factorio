pub struct TurretGraphicsSet {
    base_visualisation: TurretGraphicsSetBaseVisualisation,
}
pub enum TurretGraphicsSetBaseVisualisation {
    TurretBaseVisualisation(Box<crate::types::TurretBaseVisualisation>),
    VecTurretBaseVisualisation(Vec<crate::types::TurretBaseVisualisation>),
}
