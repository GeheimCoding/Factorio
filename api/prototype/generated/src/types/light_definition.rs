pub enum LightDefinition {
    LightDefinitionStruct(LightDefinitionStruct),
    VecLightDefinitionStruct(Vec<LightDefinitionStruct>),
}
pub struct LightDefinitionStruct {
    add_perspective: bool,
    color: Color,
    flicker_interval: u8,
    flicker_max_modifier: f32,
    flicker_min_modifier: f32,
    intensity: f32,
    minimum_darkness: f32,
    offset_flicker: bool,
    picture: Sprite,
    rotation_shift: RealOrientation,
    shift: Vector,
    size: f32,
    source_orientation_offset: RealOrientation,
    type_: LightDefinitionType,
}
pub enum LightDefinitionType {
    Basic,
    Oriented,
}
