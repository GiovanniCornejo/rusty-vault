use rand::{thread_rng, Rng};

const DEFAULT_LENGTH: std::ops::Range<usize> = 12..17;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "1234567890";
const SPECIAL_CHARS: &str = "!@#$%^&*()";

pub struct PasswordGenerator {
    pub length: usize,
    char_set: Vec<char>,
}

impl PasswordGenerator {
    pub fn new(
        pw_length: Option<usize>,
        uppercase: bool,
        lowercase: bool,
        digits: bool,
        special_chars: bool,
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

        let length = pw_length.unwrap_or(thread_rng().gen_range(DEFAULT_LENGTH));

        Self {
            length,
            char_set: char_set.chars().collect(),
        }
    }

    pub fn generate_password(&self) -> String {
        let mut pw = String::new();
        let len = self.char_set.len();

        // Generate a random character in the char_set and add to password
        for _ in 0..self.length {
            pw.push(self.char_set[thread_rng().gen_range(0..len)]);
        }
        pw
    }
}
