use crate::format::Context;

pub trait Transformation {
    fn to_pascal_case(&self) -> String;
    fn to_snake_case(&self) -> String;
    fn to_rust_type(&self, context: &Context) -> (String, Vec<String>);
}

impl Transformation for String {
    fn to_pascal_case(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut chars = self.chars();
        let next = chars
            .next()
            .expect("there should be at least one character");
        let mut pascal_case = String::new();
        if !next.is_ascii_punctuation() {
            pascal_case.push(next.to_ascii_uppercase());
        } else if let Some(next) = chars.next() {
            pascal_case.push(next.to_ascii_uppercase());
        }
        while let Some(c) = chars.next() {
            if c.is_ascii_punctuation() {
                if let Some(next) = chars.next() {
                    if !next.is_ascii_punctuation() {
                        pascal_case.push(next.to_ascii_uppercase());
                    }
                }
            } else {
                pascal_case.push(c);
            }
        }
        pascal_case
    }

    fn to_snake_case(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut chars = self.chars();
        let mut snake_case = String::from(
            chars
                .next()
                .expect("there should be at least one character")
                .to_ascii_lowercase(),
        );
        let mut with_underscore = true;
        while let Some(c) = chars.next() {
            if c.is_ascii_uppercase() || c.is_numeric() {
                if with_underscore {
                    snake_case.push_str(&format!("_{}", c.to_ascii_lowercase()));
                    with_underscore = false;
                } else {
                    snake_case.push(c.to_ascii_lowercase());
                }
            } else if c == '.' || c == '-' || c == ':' {
                snake_case.push('_');
                with_underscore = true;
            } else {
                snake_case.push(c);
                with_underscore = c != '_';
            }
        }
        snake_case
    }

    fn to_rust_type(&self, context: &Context) -> (String, Vec<String>) {
        match self.as_str() {
            "double" => (String::from("f64"), vec![]),
            "float" => (String::from("f32"), vec![]),
            "int16" => (String::from("i16"), vec![]),
            "int32" => (String::from("i32"), vec![]),
            "int64" => (String::from("i64"), vec![]),
            "int8" => (String::from("i8"), vec![]),
            "string" => (String::from("String"), vec![]),
            "uint16" => (String::from("u16"), vec![]),
            "uint32" => (String::from("u32"), vec![]),
            "uint64" => (String::from("u64"), vec![]),
            "uint8" => (String::from("u8"), vec![]),
            "type" => (String::from("type_"), vec![]),
            s => context.with_prefix(s),
        }
    }
}
