use std::io::BufRead;

fn main() {
    let mut handle = std::io::stdin().lock();
    let result = solve(&mut handle);

    println!("{}", result);
}

fn solve(reader: &mut dyn BufRead) -> i32 {
    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|bank| max_joltage_of_bank(&bank))
        .sum()
}

fn max_joltage_of_bank(bank: &str) -> i32 {
    bank.chars()
        .flat_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .fold((0, 0, 0), |(max, d1, d2), digit| {
            if d1 == -1 {
                return (digit, digit, d2);
            }
            if d2 == -1 {
                return (d1 * 10 + digit, d1, digit);
            }

            let variant1 = d1 * 10 + digit;
            let variant2 = d2 * 10 + digit;

            if variant1 <= max && variant2 <= max {
                (max, d1, d2)
            } else if variant1 > variant2 {
                (variant1, d1, digit)
            } else {
                (variant2, d2, digit)
            }
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
        assert_eq!(result, 357);
    }

    #[test]
    fn test_input_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = super::solve(&mut reader);
        assert_eq!(result, 17408);
    }

    #[rstest]
    #[case("987654321111111\n", 98)]
    #[case("811111111111119\n", 89)]
    #[case("234234234234278\n", 78)]
    #[case("818181911112111\n", 92)]
    fn test_max_joltage_of_bank(#[case] input: &str, #[case] expected: i32) {
        let result = super::max_joltage_of_bank(input);
        assert_eq!(result, expected);
    }
}
