#[derive(Debug, serde::Deserialize)]
pub struct Prototype {
    #[serde(flatten)]
    base_: crate::prototypes::PrototypeBase,
    factoriopedia_alternative: Option<String>,
}
