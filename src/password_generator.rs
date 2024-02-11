use rand::{seq::SliceRandom, thread_rng, Rng};

const DEFAULT_LENGTH: std::ops::Range<usize> = 12..17;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "1234567890";
const SPECIAL: &str = "!@#$%^&*()";

pub struct PasswordGenerator {
    length: usize,
    char_sets: Vec<Vec<char>>,
    min_uppercase: usize,
    min_lowercase: usize,
    min_digits: usize,
    min_special: usize,
}

impl PasswordGenerator {
    pub fn new(
        length: usize,
        uppercase: bool,
        lowercase: bool,
        digits: bool,
        special: bool,
        min_uppercase: usize,
        min_lowercase: usize,
        min_digits: usize,
        min_special: usize,
    ) -> Self {
        let mut char_sets = Vec::new();
        if uppercase {
            char_sets.push(UPPERCASE.chars().collect());
        }
        if lowercase {
            char_sets.push(LOWERCASE.chars().collect());
        }
        if digits {
            char_sets.push(DIGITS.chars().collect());
        }
        if special {
            char_sets.push(SPECIAL.chars().collect());
        }

        Self {
            length,
            char_sets,
            min_uppercase,
            min_lowercase,
            min_digits,
            min_special,
        }
    }

    pub fn generate_password(&self) -> String {
        let mut pw = String::new();
        let mut rng = thread_rng();

        // Keep track of minimum requirements
        let mut remaining_length = self.length;
        let required_counts = [
            self.min_uppercase,
            self.min_lowercase,
            self.min_digits,
            self.min_special,
        ];

        // Generate characters for each set until minimum requirements are met
        for (i, char_set) in self.char_sets.iter().enumerate() {
            let required_count = required_counts[i].min(remaining_length);
            for _ in 0..required_count {
                pw.push(char_set[rng.gen_range(0..char_set.len())]);
                remaining_length -= 1;
            }
        }

        // Generate remaining characters
        for _ in 0..remaining_length {
            let char_set = &self.char_sets[rng.gen_range(0..self.char_sets.len())];
            pw.push(char_set[rng.gen_range(0..char_set.len())]);
        }

        // Shuffle the generated password
        let mut pw: Vec<char> = pw.chars().collect();
        pw.shuffle(&mut rng);
        pw.iter().collect()
    }
}

pub struct PasswordGeneratorBuilder {
    length: usize,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    special: bool,
    min_uppercase: usize,
    min_lowercase: usize,
    min_digits: usize,
    min_special: usize,
}

impl PasswordGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            length: 0,
            uppercase: true,
            lowercase: true,
            digits: true,
            special: true,
            min_uppercase: 1,
            min_lowercase: 1,
            min_digits: 1,
            min_special: 1,
        }
    }

    pub fn length(mut self, length: Option<usize>) -> Self {
        self.length = length.unwrap_or(thread_rng().gen_range(DEFAULT_LENGTH));
        self
    }

    pub fn include_uppercase(mut self, uppercase: bool) -> Self {
        self.uppercase = uppercase;
        self
    }

    pub fn include_lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }

    pub fn include_digits(mut self, digits: bool) -> Self {
        self.digits = digits;
        self
    }

    pub fn include_special(mut self, special: bool) -> Self {
        self.special = special;
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

    pub fn min_special(mut self, count: usize) -> Self {
        self.min_special = count;
        self
    }

    pub fn build(self) -> PasswordGenerator {
        PasswordGenerator::new(
            self.length,
            self.uppercase,
            self.lowercase,
            self.digits,
            self.special,
            self.min_uppercase,
            self.min_lowercase,
            self.min_digits,
            self.min_special,
        )
    }
}
