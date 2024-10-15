pub enum Code {
    En,
    Pt,
}

impl Code {
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn from_string(string: &str) -> Option<Code> {
        let end = string.find('.');
        let trimmed = match end {
            Some(end) => &string[0..end],
            None => string,
        };
        println!("{}", trimmed);
        match trimmed {
            "en" => Option::Some(Code::En),
            "pt" => Option::Some(Code::Pt),
            _ => Option::Some(Code::En)
        }
    }
}

impl Clone for Code {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Code {}
