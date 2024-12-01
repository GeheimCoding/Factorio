pub type SpriteFlags = Vec<SpriteFlagsVariants>;
pub enum SpriteFlagsVariants {
    NoCrop,
    NotCompressed,
    AlwaysCompressed,
    Mipmap,
    LinearMinification,
    LinearMagnification,
    LinearMipLevel,
    AlphaMask,
    NoScale,
    Mask,
    Icon,
    Gui,
    GuiIcon,
    Light,
    Terrain,
    TerrainEffectMap,
    ReflectionEffectMap,
    Shadow,
    Smoke,
    Decal,
    LowObject,
    CorpseDecay,
    TrilinearFiltering,
    GroupNone,
    GroupTerrain,
    GroupShadow,
    GroupSmoke,
    GroupDecal,
    GroupLowObject,
    GroupGui,
    GroupIcon,
    GroupIconBackground,
    GroupEffectTexture,
}
