pub use generated::prototypes::*;

pub fn parse_prototype(input: &str) -> serde_json::Result<Prototypes> {
    serde_json::from_str(input)
}
