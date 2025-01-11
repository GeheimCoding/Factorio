pub struct SpiderLegSpecification {
    ground_position: crate::types::Vector,
    leg: crate::types::EntityID,
    leg_hit_the_ground_trigger: crate::types::TriggerEffect,
    leg_hit_the_ground_when_attacking_trigger: crate::types::TriggerEffect,
    mount_position: crate::types::Vector,
    walking_group: u8,
}