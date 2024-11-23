pub trait Case {
    fn to_pascal_case(&self) -> String;
    fn to_snake_case(&self) -> String;
}

impl Case for String {
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
            if c.is_ascii_uppercase() {
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
}
