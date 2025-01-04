pub enum ElementImageSet {
    ElementImageSet {
        base: crate::types::ElementImageSetLayer,
        glow: crate::types::ElementImageSetLayer,
        shadow: crate::types::ElementImageSetLayer,
    },
    ElementImageSetLayer(crate::types::ElementImageSetLayer),
}
