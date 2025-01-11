pub struct BuildEntityTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    build_by_dragging: bool,
    build_in_line: bool,
    consecutive: bool,
    entity: crate::types::EntityID,
    linear_power_pole_line: bool,
    match_type_only: bool,
    quality: crate::types::QualityID,
    type_: String,
}
