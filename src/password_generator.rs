use rand::{seq::SliceRandom, thread_rng, Rng};

pub const ABSOLUTE_MIN: usize = 8;
const DEFAULT_MIN: usize = 12;
const DEFAULT_MAX: usize = 16;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "1234567890";
const SPECIAL: &str = "!@#$%^&*_-()[]{}<>";

pub struct PasswordGenerator {
    length: usize,
    char_sets: Vec<Vec<char>>,
    min_counts: [usize; 4],
}

impl PasswordGenerator {
    pub fn new(
        length: Option<usize>,
        min_uppercase: usize,
        min_lowercase: usize,
        min_digits: usize,
        min_special: usize,
    ) -> Result<Self, ()> {
        let length = length.unwrap_or(
            if DEFAULT_MIN > min_uppercase + min_lowercase + min_digits + min_special {
                thread_rng().gen_range(DEFAULT_MIN..DEFAULT_MAX)
            } else {
                min_uppercase + min_lowercase + min_digits + min_special
            },
        );
        if length < ABSOLUTE_MIN
            || length < min_uppercase + min_lowercase + min_digits + min_special
        {
            eprintln!("ERROR: length of password not long enough");
            return Err(());
        }

        let char_sets = vec![
            UPPERCASE.chars().collect(),
            LOWERCASE.chars().collect(),
            DIGITS.chars().collect(),
            SPECIAL.chars().collect(),
        ];

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

    // pub fn validate_password(pw: String) -> bool {
    //     true
    // }
}

pub struct PasswordGeneratorBuilder {
    length: Option<usize>,
    min_uppercase: usize,
    min_lowercase: usize,
    min_digits: usize,
    min_special: usize,
}

impl PasswordGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            length: None,
            min_uppercase: 1,
            min_lowercase: 1,
            min_digits: 1,
            min_special: 1,
        }
    }

    pub fn length(mut self, length: Option<usize>) -> Self {
        self.length = length;
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

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_length() {
        let generator = PasswordGeneratorBuilder::new()
            .length(Some(10))
            .build()
            .unwrap();
        let password = generator.generate_password();
        assert_eq!(password.len(), 10);
    }

    #[test]
    fn test_bad_length() {
        // Test minimum length edge case
        assert!(PasswordGeneratorBuilder::new()
            .length(Some(1))
            .build()
            .is_err());
    }

    #[test]
    fn test_inclusion_of_character_sets() {
        // Test inclusion of uppercase characters
        let generator = PasswordGeneratorBuilder::new()
            .min_uppercase(1)
            .build()
            .unwrap();
        let password = generator.generate_password();
        assert!(password.chars().any(|c| c.is_ascii_uppercase()));

        // Test inclusion of lowercase characters
        let generator = PasswordGeneratorBuilder::new()
            .min_lowercase(1)
            .build()
            .unwrap();
        let password = generator.generate_password();
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));

        // Test inclusion of digits
        let generator = PasswordGeneratorBuilder::new()
            .min_digits(1)
            .build()
            .unwrap();
        let password = generator.generate_password();
        assert!(password.chars().any(|c| c.is_ascii_digit()));

        // Test inclusion of special characters
        let generator = PasswordGeneratorBuilder::new()
            .min_special(1)
            .build()
            .unwrap();
        let password = generator.generate_password();
        assert!(password.chars().any(|c| SPECIAL.contains(c)));
    }

    #[test]
    fn test_minimum_character_counts() {
        let generator = PasswordGeneratorBuilder::new()
            .length(Some(100))
            .min_uppercase(40)
            .min_lowercase(20)
            .min_digits(30)
            .min_special(10)
            .build()
            .unwrap();
        let password = generator.generate_password();
        assert_eq!(
            password.chars().filter(|&c| c.is_ascii_uppercase()).count(),
            40
        );
        assert_eq!(
            password.chars().filter(|&c| c.is_ascii_lowercase()).count(),
            20
        );
        assert_eq!(password.chars().filter(|&c| c.is_ascii_digit()).count(), 30);
        assert_eq!(
            password.chars().filter(|&c| SPECIAL.contains(c)).count(),
            10
        );
    }
}
