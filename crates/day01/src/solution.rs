pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    count_resets_to_zero(&parsed)
}

pub fn part2(input: &[String]) -> isize {
    let parsed = parse(input);
    count_passes_by_zero(&parsed)
}

fn count_resets_to_zero(input: &Vec<(bool, isize)>) -> usize {
    let mut curr = 50;
    let max = 100;
    let mut zeroes = 0;
    for (dir, length) in input {
        let actual_legth = length % max;
        match dir {
            false => {
                // going left
                if (curr - actual_legth) < 0 {
                    curr = max + (curr - actual_legth);
                } else {
                    curr -= actual_legth;
                }
            }
            true => {
                // going right
                if (curr + actual_legth) >= max {
                    curr = curr + actual_legth - max;
                } else {
                    curr += actual_legth;
                }
            }
        }
        if curr < 0 {
            unreachable!()
        }
        if curr == 0 {
            zeroes += 1;
        }
    }
    zeroes //5034, too low
}

fn count_passes_by_zero(input: &Vec<(bool, isize)>) -> isize {
    let mut curr = 50;
    let max = 100;
    let mut zeroes = 0;
    for (dir, length) in input {
        let actual_legth = length % max;
        zeroes += length.abs() / 100;
        let prev_curr = curr;
        match dir {
            false => {
                // going left
                if (curr - actual_legth) < 0 {
                    curr = max + (curr - actual_legth);
                    if curr != 0 && prev_curr != 0 {
                        zeroes += 1;
                    }
                } else {
                    curr -= actual_legth;
                }
            }
            true => {
                // going right
                if (curr + actual_legth) >= max {
                    curr = curr + actual_legth - max;
                    if curr != 0 && prev_curr != 0 {
                        zeroes += 1;
                    }
                } else {
                    curr += actual_legth;
                }
            }
        }
        if curr == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn parse(strings: &[String]) -> Vec<(bool, isize)> {
    let mut vec = Vec::new();
    for line in strings {
        let (dir_str, length_str) = line.split_at(1);
        let dir = match dir_str {
            "R" => true,
            "L" => false,
            &_ => unreachable!(),
        };
        let length = length_str.parse().unwrap();
        vec.push((dir, length));
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .lines()
        .map(|s| String::from(s.trim()))
        .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&get_input()));
    }
}
