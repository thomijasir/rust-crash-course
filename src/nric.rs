use chrono::NaiveDate;
use rand::Rng;

pub struct NRIC {
    number: String,
}

impl NRIC {
    // Generate NRIC based on birth date and residency status
    pub fn generate(birth_date: NaiveDate, is_citizen: bool) -> Self {
        let mut rng = rand::thread_rng();

        // Determine first letter based on birth year and status
        let first_letter = if is_citizen {
            if birth_date.year() >= 2000 {
                'T'
            } else {
                'S'
            }
        } else {
            if birth_date.year() >= 2000 {
                'G'
            } else {
                'F'
            }
        };

        // Generate 7 random digits
        let random_digits: String = (0..7).map(|_| rng.gen_range(0..10).to_string()).collect();

        // Calculate checksum
        let checksum = Self::calculate_checksum(first_letter, &random_digits);

        // Combine all parts
        let number = format!("{}{}{}", first_letter, random_digits, checksum);

        NRIC { number }
    }

    // Calculate the checksum letter based on the algorithm
    fn calculate_checksum(first: char, digits: &str) -> char {
        let weights = [2, 7, 6, 5, 4, 3, 2];
        let mut sum = 0;

        // Convert digits to numbers and multiply by weights
        for (i, digit) in digits.chars().enumerate() {
            sum += digit.to_digit(10).unwrap() * weights[i];
        }

        // Add offset based on first letter
        let offset = match first {
            'S' | 'T' => 0,
            'F' | 'G' => 4,
            _ => panic!("Invalid first letter"),
        };

        sum += offset;

        // Get remainder when divided by 11
        let remainder = sum % 11;

        // Maps for different first letters
        let st_map = ['J', 'Z', 'I', 'H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
        let fg_map = ['X', 'W', 'U', 'T', 'R', 'Q', 'P', 'N', 'M', 'L', 'K'];

        // Select checksum based on first letter
        match first {
            'S' | 'T' => st_map[remainder as usize],
            'F' | 'G' => fg_map[remainder as usize],
            _ => panic!("Invalid first letter"),
        }
    }

    // Get the generated NRIC number
    pub fn get_number(&self) -> &str {
        &self.number
    }

    // Validate an NRIC number
    pub fn is_valid(nric: &str) -> bool {
        if nric.len() != 9 {
            return false;
        }

        let first = nric.chars().next().unwrap();
        if !['S', 'T', 'F', 'G'].contains(&first) {
            return false;
        }

        let digits = &nric[1..8];
        if !digits.chars().all(|c| c.is_digit(10)) {
            return false;
        }

        let actual_checksum = nric.chars().last().unwrap();
        let calculated_checksum = Self::calculate_checksum(first, digits);

        actual_checksum == calculated_checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_nric_generation() {
        // Test citizen born before 2000
        let birth_date = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
        let nric = NRIC::generate(birth_date, true);
        assert!(nric.get_number().starts_with('S'));
        assert!(NRIC::is_valid(nric.get_number()));

        // Test citizen born after 2000
        let birth_date = NaiveDate::from_ymd_opt(2001, 1, 1).unwrap();
        let nric = NRIC::generate(birth_date, true);
        assert!(nric.get_number().starts_with('T'));
        assert!(NRIC::is_valid(nric.get_number()));

        // Test PR registered before 2000
        let birth_date = NaiveDate::from_ymd_opt(1990, 1, 1).unwrap();
        let nric = NRIC::generate(birth_date, false);
        assert!(nric.get_number().starts_with('F'));
        assert!(NRIC::is_valid(nric.get_number()));

        // Test PR registered after 2000
        let birth_date = NaiveDate::from_ymd_opt(2001, 1, 1).unwrap();
        let nric = NRIC::generate(birth_date, false);
        assert!(nric.get_number().starts_with('G'));
        assert!(NRIC::is_valid(nric.get_number()));
    }

    #[test]
    fn test_nric_validation() {
        // Test valid NRIC
        assert!(NRIC::is_valid("S1234567A"));

        // Test invalid length
        assert!(!NRIC::is_valid("S123456A"));

        // Test invalid first letter
        assert!(!NRIC::is_valid("X1234567A"));

        // Test invalid digits
        assert!(!NRIC::is_valid("S123A567A"));
    }
}
