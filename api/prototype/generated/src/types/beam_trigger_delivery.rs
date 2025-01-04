pub struct BeamTriggerDelivery {
    add_to_shooter: bool,
    beam: EntityID,
    destroy_with_source_or_target: bool,
    duration: u32,
    max_length: u32,
    source_offset: Vector,
    type_: Beam,
}
