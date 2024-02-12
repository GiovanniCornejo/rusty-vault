use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub const ALLOWED_MIN: usize = 13;
pub const DEFAULT_MIN: usize = 20;
pub const DEFAULT_MAX: usize = 25;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "1234567890";
const SPECIAL: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ";

// Store most common passwords directly into executable
lazy_static! {
    static ref COMMON_PASSWORDS: Vec<&'static str> =
        include_str!(r"../data/10000-most-common-passwords.txt")
            .lines()
            .collect();
}

/// Check for repeating substring patterns in a string
fn has_repeated_pattern(s: &str) -> bool {
    // Iterate through half of string
    for start_index in 0..s.len() / 2 {
        // Get the substring from the current starting index
        let remaining = &s[start_index..];
        let remaining_len = remaining.len();

        // Iterate through substrings up to half of remaining list
        for sub_len in 1..=remaining_len / 2 {
            // Form a pattern by repeating the first sub_len characters of the remaining substring
            let pattern = &remaining[..sub_len].repeat(remaining_len / sub_len);
            if pattern == remaining {
                return true;
            }
        }
    }

    false
}

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
                thread_rng().gen_range(DEFAULT_MIN..DEFAULT_MAX + 1)
            } else {
                min_uppercase + min_lowercase + min_digits + min_special
            },
        );
        if length < ALLOWED_MIN || length < min_uppercase + min_lowercase + min_digits + min_special
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

    pub fn validate_password(pw: &str) -> i32 {
        // Check if is a common password
        if COMMON_PASSWORDS.contains(&pw) {
            return -3;
        }

        // Check password length before calculating entropy
        if pw.len() < ALLOWED_MIN {
            return -2;
        }

        // Determine the size of the pool and variety of characters
        let mut character_variety = 0;
        let mut pool = String::new();
        if pw.chars().any(|c| UPPERCASE.contains(c)) {
            character_variety += 1;
            pool.push_str(UPPERCASE);
        }
        if pw.chars().any(|c| LOWERCASE.contains(c)) {
            character_variety += 1;
            pool.push_str(LOWERCASE);
        }
        if pw.chars().any(|c| DIGITS.contains(c)) {
            character_variety += 1;
            pool.push_str(DIGITS);
        }
        if pw.chars().any(|c| SPECIAL.contains(c)) {
            character_variety += 1;
            pool.push_str(SPECIAL);
        }
        let pool_size = pool.chars().collect::<std::collections::HashSet<_>>().len();

        // Calculate entropy
        let entropy = (pw.len() as f64 * (pool_size as f64).log2()) as i32;
        let mut strength = match entropy {
            e if e > 128 => 3, // Very strong
            e if e > 60 => 1,  // Strong
            e if e > 36 => 0,  // Medium
            e if e > 28 => -2, // Weak
            _ => -3,           // Very Weak
        };

        strength = match character_variety {
            4 => strength + 2,
            2 => strength + 1,
            _ => strength - 1,
        };

        // Check repeating patterns
        let has_repeating_pattern = has_repeated_pattern(pw);
        if has_repeating_pattern {
            strength -= 1;
        }

        // Clamp strength to [-2, 2]
        eprintln!(
                "Strength: {strength}\nEntropy: {entropy}\nRepeats: {has_repeating_pattern}\nLength: {length}", length = pw.len()
            );
        strength.clamp(-2, 2)
    }
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
            .length(Some(14))
            .build()
            .unwrap();
        let password = generator.generate_password();
        assert_eq!(password.len(), 14);
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

    #[test]
    fn test_repeats_in_passwords() {
        assert!(has_repeated_pattern(
            "thishasrepeatingpasswordthishasrepeatingpassword"
        ));
        assert!(!has_repeated_pattern("thishasnorepeats"));
        assert!(has_repeated_pattern("onetwoonetwo"));
        assert!(!has_repeated_pattern("onetwoone"));
        assert!(!has_repeated_pattern("racecar"));
        assert!(!has_repeated_pattern("wr#rpt#nononononopre"));
    }
}
