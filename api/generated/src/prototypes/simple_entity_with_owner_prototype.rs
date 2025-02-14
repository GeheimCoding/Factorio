#[derive(Debug, serde::Deserialize)]
pub struct SimpleEntityWithOwnerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animations: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_force_visibility")]
    force_visibility: crate::types::ForceCondition,
    lower_pictures: Option<crate::types::SpriteVariations>,
    #[serde(default = "default_lower_render_layer")]
    lower_render_layer: crate::types::RenderLayer,
    picture: Option<crate::types::Sprite4Way>,
    pictures: Option<crate::types::SpriteVariations>,
    #[serde(default = "default_random_animation_offset")]
    random_animation_offset: bool,
    #[serde(default = "default_random_variation_on_create")]
    random_variation_on_create: bool,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_secondary_draw_order")]
    secondary_draw_order: i8,
    stateless_visualisation_variations:
        Option<crate::vec::Vec<crate::types::StatelessVisualisations>>,
}
fn default_force_visibility() -> crate::types::ForceCondition {
    crate::types::ForceCondition::All
}
fn default_lower_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::LowerObject
}
fn default_random_animation_offset() -> bool {
    false
}
fn default_random_variation_on_create() -> bool {
    true
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_secondary_draw_order() -> i8 {
    0
}
