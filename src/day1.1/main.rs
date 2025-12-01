use std::io;
use std::io::{BufRead};

fn main() {
    let mut handle = io::stdin().lock();
    let result = solver(&mut handle);

    println!("{}", result);
}

fn solver(reader: &mut dyn BufRead) -> i32 {
    let mut current_sum = 50;
    let mut zero_passes = 0;

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let trimmed = content.trim();
                let (direction, value_str) = trimmed.split_at(1);

                if let Ok(value) = value_str.parse::<i32>() {
                    current_sum = match direction {
                        "R" => current_sum + value,
                        "L" => current_sum - value,
                        _ => current_sum,
                    } % 100;
                }

                if current_sum == 0 {
                    zero_passes += 1;
                }
            }
            Err(_) => continue,
        }
    }

    zero_passes
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solver_sample() {
        let input = b"L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        let mut reader: &[u8] = &input[..];
        let result = super::solver(&mut reader);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solver_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = super::solver(&mut reader);
        assert_eq!(result, 1031);
    }

    #[test]
    fn test_solver_R50_should_return_1() {
        let input = b"R50\n";
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
}