pub struct TurretGraphicsSet {
    base_visualisation: TurretGraphicsSetBaseVisualisation,
}
pub enum TurretGraphicsSetBaseVisualisation {
    TurretBaseVisualisation(Box<TurretBaseVisualisation>),
    VecTurretBaseVisualisation(Vec<TurretBaseVisualisation>),
}
