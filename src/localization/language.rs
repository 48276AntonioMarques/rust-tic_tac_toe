use crate::localization::code;

pub struct Language {
    pub code: code::Code,
    pub strings: Vec<String>,
}
