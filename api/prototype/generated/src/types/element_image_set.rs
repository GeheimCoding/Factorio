pub enum ElementImageSet {
    ElementImageSet {
        base: ElementImageSetLayer,
        glow: ElementImageSetLayer,
        shadow: ElementImageSetLayer,
    },
    ElementImageSetLayer(ElementImageSetLayer),
}
