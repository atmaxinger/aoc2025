use std::io::BufRead;

fn main() {
    let mut handle = std::io::stdin().lock();
    let result = solve(&mut handle);

    println!("{}", result);
}

fn solve(reader: &mut dyn BufRead) -> i64 {
    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|bank| max_joltage_of_bank(&bank))
        .sum()
}

fn max_joltage_of_bank(bank: &str) -> i64 {
    // For performance, reuse this vector for generating variants
    let mut temp_variant_digits = Vec::with_capacity(12);

    bank.chars()
        .flat_map(|c| c.to_digit(10))
        .map(|d| d as i64)
        .fold((0, vec![]), |(max, digits), digit| {
            if digits.len() < 2 {
                let mut new_digits = digits;
                new_digits.push(digit);
                let new_max = max * 10 + digit;
                return (new_max, new_digits);
            }

            let len = digits.len();
            let mut max_new = max;
            let mut digits_new = digits.clone();

            for cut_index in 0..len {
                temp_variant_digits.clear();
                temp_variant_digits.extend_from_slice(&digits[..cut_index]);
                temp_variant_digits.extend_from_slice(&digits[cut_index..]);

                if len < 12 {
                    temp_variant_digits.push(digit);
                } else {
                    temp_variant_digits.remove(cut_index);
                    temp_variant_digits.push(digit);
                }

                let variant = temp_variant_digits.iter().fold(0, |acc, &d| acc * 10 + d);
                if variant > max_new {
                    max_new = variant;
                    digits_new = temp_variant_digits.clone();
                }
            }

            (max_new, digits_new)
        })
        .0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn test_sample_input() {
        let input = b"987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solve(&mut reader);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_input_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = super::solve(&mut reader);
        assert_eq!(result, 172740584266849);
    }

    #[rstest]
    #[case("987654321111111\n", 987654321111)]
    #[case("811111111111119\n", 811111111119)]
    #[case("234234234234278\n", 434234234278)]
    #[case("818181911112111\n", 888911112111)]
    fn test_max_joltage_of_bank(#[case] input: &str, #[case] expected: i64) {
        let result = super::max_joltage_of_bank(input);
        assert_eq!(result, expected);
    }
}
