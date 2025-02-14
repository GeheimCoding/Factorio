#[derive(Debug, serde::Deserialize)]
pub struct Data {
    extend: crate::types::DataExtendMethod,
    is_demo: bool,
    raw: std::collections::HashMap<
        String,
        std::collections::HashMap<String, crate::types::AnyPrototype>,
    >,
}
