use sys_locale::get_locale;

pub mod code;
pub mod language;
pub mod resource;

pub struct Localization {
    pub languages: Vec<language::Language>,
}

impl Localization {
    pub fn new() -> Localization {
        Localization {
            languages: vec![
                language::Language {
                    code: code::Code::En,
                    strings: [
                        "Press any key to play again or 'x' to exit.",
                        "Now playing",
                        "Enter a number between 1 and 9.",
                        "Invalid input.",
                        "Invalid move. Square is already taken.",
                        "wins!",
                        "Draw!",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                },
                language::Language {
                    code: code::Code::Pt,
                    strings: [
                        "Clique alguma tecla ou 'x' para sair.",
                        "A jogar",
                        "Introduza um número entre 1 e 9.",
                        "Valor inválido.",
                        "Jogada inválida. Quadrado já estava ocupado.",
                        "venceu!",
                        "Empate!",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                },
            ],
        }
    }

    pub fn get_string(
        &self,
        code: code::Code,
        resource: resource::Resource,
    ) -> Result<String, String> {
        let languages = &self.languages;
        let language = languages
            .iter()
            .find(|language| language.code.to_index() == code.to_index());
        match language {
            Some(language) => {
                let strings = &language.strings;
                let size = strings.len();
                let index = resource.to_index();
                if index >= size {
                    Result::Err("Resource not found.".to_string())
                } else {
                    Result::Ok(strings[index].clone())
                }
            }
            None => Result::Err("Language not found.".to_string()),
        }
    }
}

pub fn get_localizad_string(resource: resource::Resource) -> String {
    // Get localization object
    let localization = Localization::new();
    // Get system language
    let code = get_locale().unwrap_or("en".to_string());
    match code::Code::from_string(&code) {
        Some(language) => {
            // Get localized string
            let string = localization.get_string(language, resource);
            // Print localized string
            match string {
                Ok(s) => s,
                Err(e) => e,
            }
        }
        None => "Language not found.".to_string(),
    }
}

pub fn print(resource: resource::Resource) {
    print!("{}", get_localizad_string(resource));
}
