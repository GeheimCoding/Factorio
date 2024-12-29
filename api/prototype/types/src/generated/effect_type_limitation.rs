pub enum EffectTypeLimitation {
    EffectTypeLimitationVariants(EffectTypeLimitationVariants),
    VecEffectTypeLimitationVariants(Vec<EffectTypeLimitationVariants>),
}
pub enum EffectTypeLimitationVariants {
    Speed,
    Productivity,
    Consumption,
    Pollution,
    Quality,
}
