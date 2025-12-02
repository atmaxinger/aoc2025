use std::io;
use std::io::BufRead;

fn main() {
    let mut handle = io::stdin().lock();
    let result = solver(&mut handle);

    println!("{}", result);
}

fn solver(reader: &mut dyn BufRead) -> i64 {
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| parse_line(&line))
        .flat_map(|ranges| ranges.into_iter().flat_map(|r| r.0..=r.1))
        .filter(|&id| !is_legal_id(id))
        .sum()
}

type Range = (i64, i64);

fn is_legal_id(id: i64) -> bool {
    let strid = id.to_string();

    if strid.len() % 2 != 0 {
        return true;
    }

    let mid = strid.len() / 2;
    let (first_half, second_half) = strid.split_at(mid);
    first_half != second_half
}

fn parse_line(line: &str) -> Option<Vec<Range>> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    let mut ranges = Vec::new();

    for part in parts {
        let bounds: Vec<&str> = part.split('-').collect();
        if bounds.len() != 2 {
            return None;
        }
        if let (Ok(start), Ok(end)) = (bounds[0].parse::<i64>(), bounds[1].parse::<i64>()) {
            ranges.push((start, end));
        } else {
            return None;
        }
    }

    Some(ranges)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[rstest]
    #[case(b"11-22", 33)]
    #[case(b"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124", 1227775554)]
    fn test_solver_samples(#[case] input: &[u8], #[case] expected: i64) {
        let mut reader: &[u8] = input;
        let result = super::solver(&mut reader);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solver_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = super::solver(&mut reader);
        assert_eq!(result, 26255179562);
    }

    #[test]
    fn test_parse_line_returns_correct_ranges() {
        let line = "10-20,30-40,50-60";
        let expected = vec![(10, 20), (30, 40), (50, 60)];
        let result = super::parse_line(line).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(10, true)]
    #[case(111, true)]
    #[case(11, false)]
    #[case(22, false)]
    #[case(6464, false)]
    #[case(222222, false)]
    #[case(38593859, false)]
    fn test_is_legal_id(#[case] id: i64, #[case] expected: bool) {
        let result = super::is_legal_id(id);
        assert_eq!(result, expected);
    }
}
