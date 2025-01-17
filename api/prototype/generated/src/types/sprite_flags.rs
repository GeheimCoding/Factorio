pub type SpriteFlags = Vec<SpriteFlagsVariants>;
#[derive(serde::Deserialize)]
pub enum SpriteFlagsVariants {
    #[serde(rename = "no_crop")]
    NoCrop,
    #[serde(rename = "not_compressed")]
    NotCompressed,
    #[serde(rename = "always_compressed")]
    AlwaysCompressed,
    #[serde(rename = "mipmap")]
    Mipmap,
    #[serde(rename = "linear_minification")]
    LinearMinification,
    #[serde(rename = "linear_magnification")]
    LinearMagnification,
    #[serde(rename = "linear_mip_level")]
    LinearMipLevel,
    #[serde(rename = "alpha_mask")]
    AlphaMask,
    #[serde(rename = "no_scale")]
    NoScale,
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "gui")]
    Gui,
    #[serde(rename = "gui_icon")]
    GuiIcon,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "terrain")]
    Terrain,
    #[serde(rename = "terrain_effect_map")]
    TerrainEffectMap,
    #[serde(rename = "reflection_effect_map")]
    ReflectionEffectMap,
    #[serde(rename = "shadow")]
    Shadow,
    #[serde(rename = "smoke")]
    Smoke,
    #[serde(rename = "decal")]
    Decal,
    #[serde(rename = "low_object")]
    LowObject,
    #[serde(rename = "corpse_decay")]
    CorpseDecay,
    #[serde(rename = "trilinear_filtering")]
    TrilinearFiltering,
    #[serde(rename = "group_none")]
    GroupNone,
    #[serde(rename = "group_terrain")]
    GroupTerrain,
    #[serde(rename = "group_shadow")]
    GroupShadow,
    #[serde(rename = "group_smoke")]
    GroupSmoke,
    #[serde(rename = "group_decal")]
    GroupDecal,
    #[serde(rename = "group_low_object")]
    GroupLowObject,
    #[serde(rename = "group_gui")]
    GroupGui,
    #[serde(rename = "group_icon")]
    GroupIcon,
    #[serde(rename = "group_icon_background")]
    GroupIconBackground,
    #[serde(rename = "group_effect_texture")]
    GroupEffectTexture,
}
