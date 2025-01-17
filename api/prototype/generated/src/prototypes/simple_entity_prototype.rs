#[derive(serde::Deserialize)]
pub struct SimpleEntityPrototype {
    base_: crate::prototypes::EntityWithHealthPrototype,
    animations: crate::types::AnimationVariations,
    count_as_rock_for_filtered_deconstruction: bool,
    lower_pictures: crate::types::SpriteVariations,
    lower_render_layer: crate::types::RenderLayer,
    picture: crate::types::Sprite4Way,
    pictures: crate::types::SpriteVariations,
    random_animation_offset: bool,
    random_variation_on_create: bool,
    render_layer: crate::types::RenderLayer,
    secondary_draw_order: i8,
    stateless_visualisation_variations: Vec<crate::types::StatelessVisualisations>,
}
