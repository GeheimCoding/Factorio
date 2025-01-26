#[derive(Debug, serde::Deserialize)]
pub struct Prototype {
    base_: crate::prototypes::PrototypeBase,
    factoriopedia_alternative: Option<String>,
}
