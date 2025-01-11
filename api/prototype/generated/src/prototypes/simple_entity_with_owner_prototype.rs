pub struct SimpleEntityWithOwnerPrototype {
    animations: crate::types::AnimationVariations,
    force_visibility: crate::types::ForceCondition,
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
