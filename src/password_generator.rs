use rand::Rng;
use rand::prelude::SliceRandom;

pub trait PasswordGenerator {
    fn generate_password(&self, length: usize) -> String;
}

pub struct Characters {
    pub uppercase: String,
    pub lowercase: String,
    pub numbers: String,
    pub symbols: String,
}

pub struct ComplexPasswordGenerator {
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub characters: Characters,
}

impl ComplexPasswordGenerator {
    pub fn new(include_uppercase: bool, include_lowercase: bool, include_numbers: bool, include_symbols: bool) -> Self {
        Self {
            include_uppercase,
            include_lowercase,
            include_numbers,
            include_symbols,
            characters: Characters {
                uppercase: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
                lowercase: String::from("abcdefghijklmnopqrstuvwxyz"),
                numbers: String::from("0123456789"),
                symbols: String::from("!@#$%^&*()"),
            },
        }
    }
}

impl PasswordGenerator for ComplexPasswordGenerator {
    fn generate_password(&self, length: usize) -> String {
        let mut chars = String::new();
        let mut password = String::with_capacity(length);
        let mut rng = rand::rng();

        // Ensure at least one character from each selected category
        if self.include_uppercase {
            chars.push_str(&self.characters.uppercase);
            password.push(self.characters.uppercase.chars().nth(rng.random_range(0..self.characters.uppercase.len())).unwrap());
        }
        if self.include_lowercase {
            chars.push_str(&self.characters.lowercase);
            password.push(self.characters.lowercase.chars().nth(rng.random_range(0..self.characters.lowercase.len())).unwrap());
        }
        if self.include_numbers {
            chars.push_str(&self.characters.numbers);
            password.push(self.characters.numbers.chars().nth(rng.random_range(0..self.characters.numbers.len())).unwrap());
        }
        if self.include_symbols {
            chars.push_str(&self.characters.symbols);
            password.push(self.characters.symbols.chars().nth(rng.random_range(0..self.characters.symbols.len())).unwrap());
        }

        for _ in password.len()..length {
            let idx = rng.random_range(0..chars.len());
            password.push(chars.chars().nth(idx).unwrap());
        }

        // Shuffle the password to ensure randomness
        let mut password_chars: Vec<char> = password.chars().collect();
        password_chars.shuffle(&mut rng);
        password_chars.iter().collect()
    }
}