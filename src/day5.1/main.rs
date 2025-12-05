use std::io::BufRead;
use std::ops::RangeInclusive;
use std::time::Instant;

fn main() {
    let mut handle = std::io::stdin().lock();

    let now = Instant::now();
    let result = solve(&mut handle);
    let elapsed = now.elapsed();

    println!("{}", result);
    eprintln!("Elapsed: {:.2?}", elapsed);
}

fn solve(reader: &mut dyn BufRead) -> i32 {
    let fresh_ranges = parse_fresh_ingredient_ranges(reader);

    parse_available_ingredients(reader)
        .filter(|ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
        .count() as i32
}

fn parse_fresh_ingredient_ranges(reader: &mut dyn BufRead) -> Vec<RangeInclusive<u64>> {
    let mut fresh_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for line in reader.lines().filter_map(|line| line.ok()) {
        if line.is_empty() {
            break;
        }

        match line.split_once('-') {
            Some((start, end)) => match (start.parse(), end.parse()) {
                (Ok(start), Ok(end)) => fresh_ranges.push(start..=end),
                _ => continue,
            },
            None => continue,
        };
    }

    fresh_ranges
}

fn parse_available_ingredients(reader: &mut dyn BufRead) -> impl Iterator<Item = u64> {
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.trim().parse::<u64>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";
        let mut reader = Cursor::new(input);
        let result = solve(&mut reader);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_input_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = solve(&mut reader);
        assert_eq!(result, 733);
    }

    #[test]
    fn test_parse_fresh_ingredient_ranges_no_input_returns_empty() {
        let input = "";
        let mut reader = Cursor::new(input);
        let result = parse_fresh_ingredient_ranges(&mut reader);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_parse_fresh_ingredient_ranges_with_input() {
        let input = "1-3\n5-7\n10-15\n";
        let mut reader = Cursor::new(input);
        let result = parse_fresh_ingredient_ranges(&mut reader);
        assert_eq!(result, vec![1..=3, 5..=7, 10..=15]);
    }

    #[test]
    fn test_parse_available_ingredients() {
        let input = "4\n8\n15\n16\n23\n42\n";
        let mut reader = Cursor::new(input);
        let result: Vec<u64> = parse_available_ingredients(&mut reader).collect();
        assert_eq!(result, vec![4, 8, 15, 16, 23, 42]);
    }

    #[test]
    fn test_parse_available_ingredients_with_invalid_lines() {
        let input = "4\ninvalid\n15\n16\nnot_a_number\n42\n";
        let mut reader = Cursor::new(input);
        let result: Vec<u64> = parse_available_ingredients(&mut reader).collect();
        assert_eq!(result, vec![4, 15, 16, 42]);
    }
}
