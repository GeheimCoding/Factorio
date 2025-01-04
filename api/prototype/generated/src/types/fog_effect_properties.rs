pub struct FogEffectProperties {
    color1: Color,
    color2: Color,
    detail_noise_texture: EffectTexture,
    fog_type: FogEffectPropertiesFogType,
    shape_noise_texture: EffectTexture,
}
pub enum FogEffectPropertiesFogType {
    Vulcanus,
    Gleba,
}
