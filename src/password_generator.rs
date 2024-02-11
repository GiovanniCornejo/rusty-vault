use rand::{thread_rng, Rng};

const DEFAULT_LENGTH: std::ops::Range<usize> = 12..17;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "1234567890";
const SPECIAL_CHARS: &str = "!@#$%^&*()";

pub struct PasswordGenerator {
    length: usize,
    char_set: Vec<char>,
    min_uppercase: usize,
    min_lowercase: usize,
    min_digits: usize,
    min_special_chars: usize,
}

impl PasswordGenerator {
    pub fn new(
        length: usize,
        uppercase: bool,
        lowercase: bool,
        digits: bool,
        special_chars: bool,
        min_uppercase: usize,
        min_lowercase: usize,
        min_digits: usize,
        min_special_chars: usize,
    ) -> Self {
        let mut char_set = String::new();
        if uppercase {
            char_set += UPPERCASE;
        }
        if lowercase {
            char_set += LOWERCASE;
        }
        if digits {
            char_set += DIGITS;
        }
        if special_chars {
            char_set += SPECIAL_CHARS;
        }

        Self {
            length,
            char_set: char_set.chars().collect(),
            min_uppercase,
            min_lowercase,
            min_digits,
            min_special_chars,
        }
    }

    pub fn generate_password(&self) -> String {
        let mut pw = String::new();
        let mut rng = thread_rng();
        let len = self.char_set.len();

        // Generate a random character in the char_set and add to password
        for _ in 0..self.length {
            pw.push(self.char_set[rng.gen_range(0..len)]);
        }
        pw
    }
}

pub struct PasswordGeneratorBuilder {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    special_chars: bool,
    min_uppercase: usize,
    min_lowercase: usize,
    min_digits: usize,
    min_special_chars: usize,
}

impl PasswordGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            length: 0,
            uppercase: true,
            lowercase: true,
            digits: true,
            special_chars: true,
            min_uppercase: 1,
            min_lowercase: 1,
            min_digits: 1,
            min_special_chars: 1,
        }
    }

    pub fn length(mut self, length: Option<usize>) -> Self {
        self.length = length.unwrap_or(thread_rng().gen_range(DEFAULT_LENGTH));
        self
    }

    pub fn include_uppercase(mut self, include: bool) -> Self {
        self.uppercase = include;
        self
    }

    pub fn include_lowercase(mut self, include: bool) -> Self {
        self.lowercase = include;
        self
    }

    pub fn include_digits(mut self, include: bool) -> Self {
        self.digits = include;
        self
    }

    pub fn include_special_chars(mut self, include: bool) -> Self {
        self.special_chars = include;
        self
    }

    pub fn min_uppercase(mut self, count: usize) -> Self {
        self.min_uppercase = count;
        self
    }

    pub fn min_lowercase(mut self, count: usize) -> Self {
        self.min_lowercase = count;
        self
    }

    pub fn min_digits(mut self, count: usize) -> Self {
        self.min_digits = count;
        self
    }

    pub fn min_special_chars(mut self, count: usize) -> Self {
        self.min_special_chars = count;
        self
    }

    pub fn build(self) -> PasswordGenerator {
        PasswordGenerator::new(
            self.length,
            self.uppercase,
            self.lowercase,
            self.digits,
            self.special_chars,
            self.min_uppercase,
            self.min_lowercase,
            self.min_digits,
            self.min_special_chars,
        )
    }
}
