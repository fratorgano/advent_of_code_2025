use helper::matrix;

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    count_accessible_rolls(&parsed)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    remove_all_possible(parsed)
}

pub fn count_accessible_rolls(matrix: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            let neighbours = matrix::find_neighbours_indices_including_diagonals(matrix, (r, c));
            if matrix[r][c] == '@' {
                let adjacent_rolls = neighbours
                    .iter()
                    .filter(|n| matrix[n.0][n.1] == '@')
                    .count();
                if adjacent_rolls < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn get_accessible_rolls(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut accessible_rolls = vec![];
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            let neighbours = matrix::find_neighbours_indices_including_diagonals(matrix, (r, c));
            if matrix[r][c] == '@' {
                let adjacent_rolls = neighbours
                    .iter()
                    .filter(|n| matrix[n.0][n.1] == '@')
                    .count();
                if adjacent_rolls < 4 {
                    accessible_rolls.push((r, c));
                }
            }
        }
    }
    accessible_rolls
}

pub fn remove_all_possible(matrix: Vec<Vec<char>>) -> usize {
    let mut total_removed_rolls = 0;
    let mut removed_rolls = 1;
    let mut matrix = matrix;
    while removed_rolls > 0 {
        let accessible_rolls = get_accessible_rolls(&matrix);
        removed_rolls = accessible_rolls.len();
        total_removed_rolls += removed_rolls;
        for (r, c) in accessible_rolls {
            matrix[r][c] = '.'
        }
    }
    total_removed_rolls
}

fn parse(strings: &[String]) -> Vec<Vec<char>> {
    let mut matrix = vec![];
    for line in strings {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(43, part2(&get_input()));
    }
}
