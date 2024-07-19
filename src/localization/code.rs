pub enum Code {
    En,
    Pt,
}

impl Code {
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn from_string(string: &str) -> Option<Code> {
        match string {
            "en" => Option::Some(Code::En),
            "pt" => Option::Some(Code::Pt),
            "pt-BR" => Option::Some(Code::Pt),
            "pt-PT" => Option::Some(Code::Pt),
            _ => Option::None,
        }
    }
}

impl Clone for Code {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Code {}
