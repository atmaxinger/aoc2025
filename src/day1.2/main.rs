use std::cmp::max;
use std::io;
use std::io::BufRead;

fn main() {
    let mut handle = io::stdin().lock();
    let result = solver(&mut handle);

    println!("{}", result);
}

fn parse_instruction(instruction: &str) -> Option<i32> {
    let trimmed = instruction.trim();
    let (direction, value_str) = trimmed.split_at(1);
    match value_str.parse::<i32>() {
        Ok(v) => match direction {
            "R" => Some(v),
            "L" => Some(-v),
            _ => None,
        },
        Err(_) => None,
    }
}

fn solver(reader: &mut dyn BufRead) -> i32 {
    let mut current_sum = 50;
    let mut zero_passes = 0;

    for instructions in reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| parse_instruction(&line))
    {
        let was_zero = current_sum == 0;
        current_sum = current_sum + instructions;

        zero_passes += match current_sum {
            ..=0 => max(1, (current_sum.abs() / 100) + 1) - if was_zero { 1 } else { 0 },
            100.. => current_sum / 100,
            _ => 0,
        };

        current_sum = current_sum.rem_euclid(100);
    }

    zero_passes
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(b"L68\n", 1)]
    #[case(b"L68\nL30\n", 1)]
    #[case(b"L68\nL30\nR48\n", 2)]
    #[case(b"L68\nL30\nR48\nL5\n", 2)]
    #[case(b"L68\nL30\nR48\nL5\nR60\n", 3)]
    #[case(b"L68\nL30\nR48\nL5\nR60\nL55", 4)]
    #[case(b"L68\nL30\nR48\nL5\nR60\nL55\nL1\n", 4)]
    #[case(b"L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\n", 5)]
    #[case(b"L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\n", 5)]
    #[case(b"L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n", 6)]
    fn test_solver_samples(#[case] input: &[u8], #[case] expected: i32) {
        let mut reader: &[u8] = input;
        let result = super::solver(&mut reader);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solver_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = super::solver(&mut reader);
        assert_eq!(result, 5831);
    }

    #[test]
    fn test_solver_R51_should_return_1() {
        let input = b"R51\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_solver_R50_should_return_1() {
        let input = b"R50\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_solver_L51_should_return_1() {
        let input = b"L51\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_solver_L50_should_return_1() {
        let input = b"L50\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_solver_R100_should_return_1() {
        let input = b"L100\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_solver_R150_should_return_2() {
        let input = b"L150\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solver_R1000_should_return_10() {
        let input = b"L1000\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 10);
    }
}
