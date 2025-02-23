use rand::Rng;
use std::fs;

pub trait PasswordGenerator {
    fn generate_password(&self, length: usize) -> String;
}

pub struct ComplexPasswordGenerator {
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub characters: Characters,
}

#[derive(serde::Deserialize)]
pub struct Characters {
    pub uppercase: String,
    pub lowercase: String,
    pub numbers: String,
    pub symbols: String,
}

impl ComplexPasswordGenerator {
    pub fn new(include_uppercase: bool, include_lowercase: bool, include_numbers: bool, include_symbols: bool) -> Self {
        let config: Config = toml::from_str(&fs::read_to_string("config.toml").expect("Failed to read config file")).expect("Failed to parse config file");
        Self {
            include_uppercase,
            include_lowercase,
            include_numbers,
            include_symbols,
            characters: config.characters,
        }
    }
}

#[derive(serde::Deserialize)]
struct Config {
    characters: Characters,
}

impl PasswordGenerator for ComplexPasswordGenerator {
    fn generate_password(&self, length: usize) -> String {
        let mut chars = String::new();
        if self.include_uppercase {
            chars.push_str(&self.characters.uppercase);
        }
        if self.include_lowercase {
            chars.push_str(&self.characters.lowercase);
        }
        if self.include_numbers {
            chars.push_str(&self.characters.numbers);
        }
        if self.include_symbols {
            chars.push_str(&self.characters.symbols);
        }

        let mut rng = rand::rng();
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect()
    }
}