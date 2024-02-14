use lazy_static::lazy_static;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub const ALLOWED_MIN: usize = 13;
pub const DEFAULT_MIN: usize = 20;
pub const DEFAULT_MAX: usize = 25;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxz";
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

struct CharSet {
    chars: Vec<char>,
    min_count: usize,
}

pub struct PasswordGenerator {
    length: usize,
    char_sets: Vec<CharSet>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        let length = thread_rng().gen_range(DEFAULT_MIN..DEFAULT_MAX + 1);

        let char_sets = vec![
            CharSet {
                chars: UPPERCASE.chars().collect(),
                min_count: 1,
            },
            CharSet {
                chars: LOWERCASE.chars().collect(),
                min_count: 1,
            },
            CharSet {
                chars: DIGITS.chars().collect(),
                min_count: 1,
            },
            CharSet {
                chars: SPECIAL.chars().collect(),
                min_count: 1,
            },
        ];

        Self { length, char_sets }
    }

    pub fn length(&mut self, length: usize) -> Result<(), ()> {
        if length < ALLOWED_MIN {
            eprintln!("ERROR: length must be at least {ALLOWED_MIN}");
            return Err(());
        }

        self.length = length;
        Ok(())
    }

    fn update_length(&mut self) {
        let min = self.char_sets[0].min_count
            + self.char_sets[1].min_count
            + self.char_sets[2].min_count
            + self.char_sets[3].min_count;
        if self.length < min {
            self.length = min;
        }
    }

    pub fn min_upper(&mut self, count: usize) -> Result<(), ()> {
        if count < 1 {
            eprintln!("ERROR: minimum uppercase must be at least 1");
            return Err(());
        }

        self.char_sets[0].min_count = count;
        self.update_length();
        Ok(())
    }

    pub fn min_lower(&mut self, count: usize) -> Result<(), ()> {
        if count < 1 {
            eprintln!("ERROR: minimum lowercase must be at least 1");
            return Err(());
        }

        self.char_sets[1].min_count = count;
        self.update_length();
        Ok(())
    }

    pub fn min_digits(&mut self, count: usize) -> Result<(), ()> {
        if count < 1 {
            eprintln!("ERROR: minimum digits must be at least 1");
            return Err(());
        }

        self.char_sets[2].min_count = count;
        self.update_length();
        Ok(())
    }

    pub fn min_special(&mut self, count: usize) -> Result<(), ()> {
        if count < 1 {
            eprintln!("ERROR: minimum special characters must be at least 1");
            return Err(());
        }

        self.char_sets[3].min_count = count;
        self.update_length();

        Ok(())
    }

    pub fn generate_password(&self) -> String {
        let mut pw = String::new();
        let mut rng = thread_rng();

        // Keep track of minimum requirements
        let mut remaining_length = self.length;

        // Generate characters for each set until minimum requirements are met
        for (i, char_set) in self.char_sets.iter().enumerate() {
            let required_count = self.char_sets[i].min_count.min(remaining_length);

            for _ in 0..required_count {
                pw.push(char_set.chars[rng.gen_range(0..char_set.chars.len())]);
                remaining_length -= 1;
            }
        }

        // Generate remaining characters
        for _ in 0..remaining_length {
            let char_set = &self.char_sets[rng.gen_range(0..self.char_sets.len())];

            pw.push(char_set.chars[rng.gen_range(0..char_set.chars.len())]);
        }

        // Shuffle the generated password
        let mut pw: Vec<char> = pw.chars().collect();
        pw.shuffle(&mut rng);
        pw.iter().collect()
    }

    pub fn validate_password(pw: &str, allow_short: bool) -> i32 {
        // Check for allowable password length
        if !allow_short && pw.len() < ALLOWED_MIN {
            return -2;
        }

        // Check if is a common password
        if COMMON_PASSWORDS.contains(&pw) {
            return -3;
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

        // Check smaller lengths
        if pw.len() <= 8 {
            strength -= 4;
        } else if pw.len() <= 10 {
            strength -= 3;
        } else if pw.len() <= 13 {
            strength -= 2;
        }

        // Clamp strength to [-2, 2]
        strength.clamp(-2, 2)
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
        let mut pg = PasswordGenerator::new();
        pg.length(14).unwrap();
        let pw = pg.generate_password();
        assert_eq!(pw.len(), 14);

        let mut pg = PasswordGenerator::new();
        pg.length(14).unwrap();
        let pw = pg.generate_password();
        assert_eq!(pw.len(), 30);
    }

    #[test]
    fn test_bad_length() {
        let mut pg = PasswordGenerator::new();
        assert!(pg.length(1).is_err());

        let mut pg = PasswordGenerator::new();
        assert!(pg.length(12).is_err());
    }

    #[test]
    fn test_inclusion_of_character_sets() {
        let pg = PasswordGenerator::new();
        let pw = pg.generate_password();
        assert!(pw.chars().any(|c| c.is_ascii_uppercase()));
        assert!(pw.chars().any(|c| c.is_ascii_lowercase()));
        assert!(pw.chars().any(|c| c.is_ascii_digit()));
        assert!(pw.chars().any(|c| SPECIAL.contains(c)));
    }

    #[test]
    fn test_minimum_character_counts() {
        let mut pg = PasswordGenerator::new();
        pg.length(100).unwrap();
        pg.min_upper(40).unwrap();
        pg.min_lower(20).unwrap();
        pg.min_digits(30).unwrap();
        pg.min_special(10).unwrap();

        let pw = pg.generate_password();
        assert_eq!(pw.chars().filter(|&c| c.is_ascii_uppercase()).count(), 40);
        assert_eq!(pw.chars().filter(|&c| c.is_ascii_lowercase()).count(), 20);
        assert_eq!(pw.chars().filter(|&c| c.is_ascii_digit()).count(), 30);
        assert_eq!(pw.chars().filter(|&c| SPECIAL.contains(c)).count(), 10);
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
