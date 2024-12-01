pub struct TurretGraphicsSet {
    base_visualisation: TurretGraphicsSetBaseVisualisation,
}
pub enum TurretGraphicsSetBaseVisualisation {
    TurretBaseVisualisation(TurretBaseVisualisation),
    VecTurretBaseVisualisation(Vec<TurretBaseVisualisation>),
}
