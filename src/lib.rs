pub struct FirstDigitCalculator {
    last_n: u64,
    last_power: u64,
}

impl FirstDigitCalculator {
    pub fn new() -> Self {
        FirstDigitCalculator {
            last_n: 0,
            last_power: 1,
        }
    }

    pub fn first_digit(&mut self, n: u64) -> u64 {
        if n < self.last_n {
            // Reset if n is smaller than the last number
            self.last_n = 0;
            self.last_power = 1;
        }

        while self.last_power * 10 <= n {
            self.last_power *= 10;
        }

        let mut result = n / self.last_power;

        // One more division if needed
        if result >= 10 {
            result /= 10;
            self.last_power *= 10;
        }

        self.last_n = n;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit() {
        let mut calculator = FirstDigitCalculator::new();
        assert_eq!(calculator.first_digit(0), 0);
        assert_eq!(calculator.first_digit(5), 5);
        assert_eq!(calculator.first_digit(9), 9);
    }

    #[test]
    fn test_multiple_digits() {
        let mut calculator = FirstDigitCalculator::new();
        assert_eq!(calculator.first_digit(10), 1);
        assert_eq!(calculator.first_digit(23), 2);
        assert_eq!(calculator.first_digit(456), 4);
        assert_eq!(calculator.first_digit(7890), 7);
    }

    #[test]
    fn test_increasing_sequence() {
        let mut calculator = FirstDigitCalculator::new();
        assert_eq!(calculator.first_digit(1), 1);
        assert_eq!(calculator.first_digit(12), 1);
        assert_eq!(calculator.first_digit(123), 1);
        assert_eq!(calculator.first_digit(1234), 1);
        assert_eq!(calculator.first_digit(12345), 1);
    }

    #[test]
    fn test_decreasing_sequence() {
        let mut calculator = FirstDigitCalculator::new();
        assert_eq!(calculator.first_digit(98765), 9);
        assert_eq!(calculator.first_digit(8765), 8);
        assert_eq!(calculator.first_digit(765), 7);
        assert_eq!(calculator.first_digit(65), 6);
        assert_eq!(calculator.first_digit(5), 5);
    }

    #[test]
    fn test_mixed_sequence() {
        let mut calculator = FirstDigitCalculator::new();
        assert_eq!(calculator.first_digit(100), 1);
        assert_eq!(calculator.first_digit(2000), 2);
        assert_eq!(calculator.first_digit(30), 3);
        assert_eq!(calculator.first_digit(4000), 4);
        assert_eq!(calculator.first_digit(50), 5);
    }

    #[test]
    fn test_large_numbers() {
        let mut calculator = FirstDigitCalculator::new();
        assert_eq!(calculator.first_digit(1_000_000_000), 1);
        assert_eq!(calculator.first_digit(9_876_543_210), 9);
    }

    #[test]
    fn test_edge_cases() {
        let mut calculator = FirstDigitCalculator::new();
        assert_eq!(calculator.first_digit(999), 9);
        assert_eq!(calculator.first_digit(1000), 1);
        assert_eq!(calculator.first_digit(1001), 1);
    }
}
