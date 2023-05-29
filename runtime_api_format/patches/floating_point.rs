#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FloatingPoint<T> {
    SpecialValue(String),
    Value(T),
}

type Float = FloatingPoint<f32>;
type Double = FloatingPoint<f64>;
