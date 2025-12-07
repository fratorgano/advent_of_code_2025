pub fn part1(input: &[String]) -> usize {
    let (values, operators) = parse(input);
    do_homework(values, operators)
}

pub fn part2(input: &[String]) -> usize {
    let (values, operators) = parse2(input);
    do_homework2(values, operators)
}

fn do_homework(values: Vec<Vec<usize>>, operators: Vec<char>) -> usize {
    let mut total = 0;
    for j in 0..values[0].len() {
        let mut result = 0;
        let op = operators[j];
        if op == '*' {
            result = 1;
        }

        for i in 0..values.len() {
            match op {
                '+' => result += values[i][j],
                '*' => result *= values[i][j],
                _ => unreachable!(),
            }
        }
        total += result;
    }
    total
}

fn do_homework2(values: Vec<Vec<usize>>, operators: Vec<char>) -> usize {
    let mut total = 0;
    for i in 0..values.len() {
        match operators[i] {
            '+' => total += values[i].iter().fold(0, |acc, x| acc + x),
            '*' => total += values[i].iter().fold(1, |acc, x| acc * x),
            _ => unreachable!(),
        }
    }
    total
}

fn parse(strings: &[String]) -> (Vec<Vec<usize>>, Vec<char>) {
    let mut values = vec![];
    for line in &strings[..strings.len() - 1] {
        let parts = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        values.push(parts);
    }
    let operators = strings
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect();
    (values, operators)
}

fn parse2(strings: &[String]) -> (Vec<Vec<usize>>, Vec<char>) {
    let char_matrix: Vec<Vec<char>> = strings.iter().map(|x| x.chars().collect()).collect();
    let mut values: Vec<Vec<usize>> = vec![];
    let mut operators = vec![];
    let mut group_values: Vec<usize> = vec![];
    let mut op = None;
    for c in 0..char_matrix[0].len() {
        let mut value = vec![];
        for r in 0..char_matrix.len() {
            if char_matrix[r][c].is_numeric() {
                value.push(char_matrix[r][c]);
            } else if !char_matrix[r][c].is_whitespace() {
                op = Some(char_matrix[r][c])
            }
        }

        if let Some(operator) = op {
            operators.push(operator);
            op = None;
        }
        if value.is_empty() {
            values.push(group_values.clone());
            group_values = vec![];
        } else {
            group_values.push(value.to_vec().iter().collect::<String>().parse().unwrap());
        }
    }
    values.push(group_values.clone());

    (values, operators)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .lines()
            .map(|s| String::from(s))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(4277556, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3263827, part2(&get_input()));
    }
}
