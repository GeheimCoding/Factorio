pub struct SpiderLegSpecification {
    ground_position: Vector,
    leg: EntityID,
    leg_hit_the_ground_trigger: TriggerEffect,
    leg_hit_the_ground_when_attacking_trigger: TriggerEffect,
    mount_position: Vector,
    walking_group: u8,
}
