pub trait Transformation {
    fn to_pascal_case(&self) -> String;
    fn to_snake_case(&self) -> String;
    fn to_rust_type(&self) -> String;
}

impl Transformation for String {
    fn to_pascal_case(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut chars = self.chars();
        let mut pascal_case = String::from(
            chars
                .next()
                .expect("there should be at least one character")
                .to_ascii_uppercase(),
        );
        while let Some(c) = chars.next() {
            if c == '_' || c == '.' || c == '-' || c == ':' {
                if let Some(next) = chars.next() {
                    pascal_case.push(next.to_ascii_uppercase());
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
                with_underscore = true;
            }
        }
        snake_case
    }

    fn to_rust_type(&self) -> String {
        match self.as_str() {
            "double" => "f64".to_owned(),
            "float" => "f32".to_owned(),
            "int16" => "i16".to_owned(),
            "int32" => "i32".to_owned(),
            "int64" => "i64".to_owned(),
            "int8" => "i8".to_owned(),
            "string" => "String".to_owned(),
            "uint16" => "u16".to_owned(),
            "uint32" => "u32".to_owned(),
            "uint64" => "u64".to_owned(),
            "uint8" => "u8".to_owned(),
            s => s.to_owned(),
        }
    }
}
