#[derive(Debug, serde::Deserialize)]
pub struct Loader1x1Prototype {
    #[serde(flatten)]
    base_: crate::prototypes::LoaderPrototype,
}
