#[derive(Debug, serde::Deserialize)]
pub struct Loader1x2Prototype {
    #[serde(flatten)]
    base_: crate::prototypes::LoaderPrototype,
}
