pub type SpriteFlags = Vec<SpriteFlagsVariants>;
#[derive(Debug, serde::Deserialize)]
pub enum SpriteFlagsVariants {
    #[serde(rename = "no-crop")]
    NoCrop,
    #[serde(rename = "not-compressed")]
    NotCompressed,
    #[serde(rename = "always-compressed")]
    AlwaysCompressed,
    #[serde(rename = "mipmap")]
    Mipmap,
    #[serde(rename = "linear-minification")]
    LinearMinification,
    #[serde(rename = "linear-magnification")]
    LinearMagnification,
    #[serde(rename = "linear-mip-level")]
    LinearMipLevel,
    #[serde(rename = "alpha-mask")]
    AlphaMask,
    #[serde(rename = "no-scale")]
    NoScale,
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "gui")]
    Gui,
    #[serde(rename = "gui-icon")]
    GuiIcon,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "terrain")]
    Terrain,
    #[serde(rename = "terrain-effect-map")]
    TerrainEffectMap,
    #[serde(rename = "reflection-effect-map")]
    ReflectionEffectMap,
    #[serde(rename = "shadow")]
    Shadow,
    #[serde(rename = "smoke")]
    Smoke,
    #[serde(rename = "decal")]
    Decal,
    #[serde(rename = "low-object")]
    LowObject,
    #[serde(rename = "corpse-decay")]
    CorpseDecay,
    #[serde(rename = "trilinear-filtering")]
    TrilinearFiltering,
    #[serde(rename = "group=none")]
    GroupNone,
    #[serde(rename = "group=terrain")]
    GroupTerrain,
    #[serde(rename = "group=shadow")]
    GroupShadow,
    #[serde(rename = "group=smoke")]
    GroupSmoke,
    #[serde(rename = "group=decal")]
    GroupDecal,
    #[serde(rename = "group=low-object")]
    GroupLowObject,
    #[serde(rename = "group=gui")]
    GroupGui,
    #[serde(rename = "group=icon")]
    GroupIcon,
    #[serde(rename = "group=icon-background")]
    GroupIconBackground,
    #[serde(rename = "group=effect-texture")]
    GroupEffectTexture,
}
