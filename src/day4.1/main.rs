use std::io::BufRead;

fn main() {
    let mut handle = std::io::stdin().lock();
    let result = solve(&mut handle);

    println!("{}", result);
}

struct Grid {
    cells: Vec<char>,
    rows: usize,
    cols: usize,
}

fn solve(reader: &mut dyn BufRead) -> i32 {
    let grid = parse_grid(reader);

    let mut total_neighbors = 0;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if let Some(count) = grid.get_neighbors(row, col) && count < 4 {
                total_neighbors += 1;
            }
        }
    }

    total_neighbors
}

fn parse_grid(reader: &mut dyn BufRead) -> Grid {
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    let grid: Vec<char> = lines
        .iter()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    Grid {
        cells: grid,
        rows: lines.len(),
        cols: lines[0].len(),
    }
}

impl Grid {
    fn get_neighbors(&self, row: usize, col: usize) -> Option<i32> {
        let directions: [(isize,isize);8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        match self.cells.get(row * self.cols + col) {
            None => None,
            Some('.') => None,
            _ => {
                let mut neighbors = 0;
                for (dc, dr) in directions.iter() {
                    let new_row = row as isize + dr;
                    let new_col = col as isize + dc;
                    if new_row >= 0 && new_row < self.rows as isize
                        && new_col >= 0 && new_col < self.cols as isize
                        && self.cells[new_row as usize * self.cols + new_col as usize] == '@' {
                        neighbors += 1;
                    }
                }

                Some(neighbors)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn test_sample() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let mut reader = std::io::Cursor::new(input);
        let result = super::solve(&mut reader);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_input_from_file() {
        let input = include_str!("input.txt");
        let mut reader = input.as_bytes();
        let result = super::solve(&mut reader);
        assert_eq!(result, 1457);
    }

    #[test]
    fn test_parse_grid() {
        let input = "..@.\n@@..\n.@.@\n";
        let mut reader = std::io::Cursor::new(input);
        let grid = super::parse_grid(&mut reader);
        assert_eq!(grid.rows, 3);
        assert_eq!(grid.cols, 4);
        assert_eq!(grid.cells, vec!['.', '.', '@', '.', '@', '@', '.', '.', '.', '@', '.', '@']);
    }

    #[rstest]
    #[case(b"..@.\n@@..\n.@.@\n", 2, 0, None)] // cell is '.', has no neighbors
    #[case(b"..@.\n@@..\n.@.@\n", 2, 1, Some(2))]
    fn test_get_neighbors(
        #[case] input: &[u8],
        #[case] row: usize,
        #[case] col: usize,
        #[case] expected: Option<i32>,
    ) {
        let mut reader = std::io::Cursor::new(input);
        let grid = super::parse_grid(&mut reader);
        let neighbors = grid.get_neighbors(row, col);
        assert_eq!(neighbors, expected);
    }
}