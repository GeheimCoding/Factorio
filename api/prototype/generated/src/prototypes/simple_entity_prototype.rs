#[derive(serde::Deserialize)]
pub struct SimpleEntityPrototype {
    base_: crate::prototypes::EntityWithHealthPrototype,
    animations: crate::types::AnimationVariations,
    #[serde(default = "default_count_as_rock_for_filtered_deconstruction")]
    count_as_rock_for_filtered_deconstruction: bool,
    lower_pictures: crate::types::SpriteVariations,
    #[serde(default = "default_lower_render_layer")]
    lower_render_layer: crate::types::RenderLayer,
    picture: crate::types::Sprite4Way,
    pictures: crate::types::SpriteVariations,
    #[serde(default = "default_random_animation_offset")]
    random_animation_offset: bool,
    #[serde(default = "default_random_variation_on_create")]
    random_variation_on_create: bool,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_secondary_draw_order")]
    secondary_draw_order: i8,
    stateless_visualisation_variations: Vec<crate::types::StatelessVisualisations>,
}
fn default_count_as_rock_for_filtered_deconstruction() -> bool {
    false
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
