#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MaybeCycle<T> {
    Cycle { cycle_id: u32 },
    Value(Box<T>),
}
