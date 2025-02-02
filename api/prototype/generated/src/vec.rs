#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Vec<T> {
    Vec(std::vec::Vec<T>),
    Empty {},
}

impl<T> From<Vec<T>> for std::vec::Vec<T> {
    fn from(value: Vec<T>) -> Self {
        match value {
            Vec::Vec(vec) => vec,
            Vec::Empty { .. } => vec![],
        }
    }
}
