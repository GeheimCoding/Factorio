pub use generated::types::*;

pub trait ParseType {
    fn parse(input: &str) -> serde_json::Result<Self>
    where
        Self: Sized;
}

impl ParseType for Types {
    fn parse(input: &str) -> serde_json::Result<Self> {
        serde_json::from_str(input)
    }
}
