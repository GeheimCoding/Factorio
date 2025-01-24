pub use generated::prototypes::*;

pub trait ParsePrototype {
    fn parse(input: &str) -> serde_json::Result<Self>
    where
        Self: Sized;
}

impl ParsePrototype for Prototypes {
    fn parse(input: &str) -> serde_json::Result<Self> {
        serde_json::from_str(input)
    }
}
