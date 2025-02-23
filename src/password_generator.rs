use rand::Rng;

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
        let mut password = String::with_capacity(length);

        for _ in 0..length {
            let idx = rng.random_range(0..chars.len());
            password.push(chars.as_bytes()[idx] as char);
        }

        password
    }
}