pub struct CreateParticleTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    apply_tile_tint: CreateParticleTriggerEffectItemApplyTileTint,
    frame_speed: f32,
    frame_speed_deviation: f32,
    initial_height: f32,
    initial_height_deviation: f32,
    initial_vertical_speed: f32,
    initial_vertical_speed_deviation: f32,
    movement_multiplier: f32,
    offset_deviation: crate::types::SimpleBoundingBox,
    offsets: Vec<crate::types::Vector>,
    only_when_visible: bool,
    particle_name: crate::types::ParticleID,
    rotate_offsets: bool,
    show_in_tooltip: bool,
    speed_from_center: f32,
    speed_from_center_deviation: f32,
    tail_length: u8,
    tail_length_deviation: u8,
    tail_width: f32,
    tile_collision_mask: crate::types::CollisionMaskConnector,
    tint: crate::types::Color,
    type_: String,
}
pub enum CreateParticleTriggerEffectItemApplyTileTint {
    Primary,
    Secondary,
}
