use rand::{seq::SliceRandom, thread_rng, Rng};

const DEFAULT_LENGTH: std::ops::Range<usize> = 12..17;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "1234567890";
const SPECIAL: &str = "!@#$%^&*()";

pub struct PasswordGenerator {
    length: usize,
    char_sets: Vec<Vec<char>>,
    min_counts: [usize; 4],
}

impl PasswordGenerator {
    pub fn new(
        length: usize,
        min_uppercase: usize,
        min_lowercase: usize,
        min_digits: usize,
        min_special: usize,
    ) -> Result<Self, ()> {
        if length < min_uppercase + min_lowercase + min_digits + min_special {
            eprintln!(
                "ERROR: length of password cannot be lower than minimum requirements: {}",
                min_uppercase + min_lowercase + min_digits + min_special
            );
            return Err(());
        }

        let mut char_sets = Vec::new();
        char_sets.push(UPPERCASE.chars().collect());
        char_sets.push(LOWERCASE.chars().collect());
        char_sets.push(DIGITS.chars().collect());
        char_sets.push(SPECIAL.chars().collect());

        let min_counts = [min_uppercase, min_lowercase, min_digits, min_special];

        Ok(Self {
            length,
            char_sets,
            min_counts,
        })
    }

    pub fn generate_password(&self) -> String {
        let mut pw = String::new();
        let mut rng = thread_rng();

        // Keep track of minimum requirements
        let mut remaining_length = self.length;

        // Generate characters for each set until minimum requirements are met
        for (i, char_set) in self.char_sets.iter().enumerate() {
            let required_count = self.min_counts[i].min(remaining_length);
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
    min_uppercase: usize,
    min_lowercase: usize,
    min_digits: usize,
    min_special: usize,
}

impl PasswordGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            length: 0,

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

    pub fn build(self) -> Result<PasswordGenerator, ()> {
        PasswordGenerator::new(
            self.length,
            self.min_uppercase,
            self.min_lowercase,
            self.min_digits,
            self.min_special,
        )
    }
}
