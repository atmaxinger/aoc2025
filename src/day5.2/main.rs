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

fn solve(reader: &mut dyn BufRead) -> u64 {
    let fresh_ranges = parse_fresh_ingredient_ranges(reader);
    let merged_ranges: Vec<RangeInclusive<u64>> = merge_ranges(&fresh_ranges);

    merged_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn merge_ranges(ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|r| *r.start());

    let mut merged = vec![sorted[0].clone()];

    for range in sorted.iter().skip(1) {
        let last = merged.last_mut().unwrap();

        if *range.start() <= *last.end() + 1 {
            *last = *last.start()..=(*last.end()).max(*range.end());
        } else {
            merged.push(range.clone());
        }
    }

    merged
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";
        let mut reader = Cursor::new(input);
        let result = solve(&mut reader);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_input_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = solve(&mut reader);
        assert_eq!(result, 345821388687084);
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
    fn test_merge_ranges() {
        let ranges = vec![1..=3, 2..=5, 10..=12, 11..=15];
        let result = merge_ranges(&ranges);
        assert_eq!(result, vec![1..=5, 10..=15]);
    }
}
