use std::vec;

pub fn part1(input: &[String]) -> usize {
    find_max_jolts(input, 2)
}

pub fn part2(input: &[String]) -> usize {
    find_max_jolts(input, 12)
}

fn find_max_jolts(strings: &[String], digits: usize) -> usize {
    let mut total_jolts = 0;
    for line in strings {
        let mut jolts: Vec<char> = vec![];
        let chars: Vec<char> = line.chars().collect();
        let mut index = 0;
        while jolts.len() < digits {
            let (max, max_index) = find_valid_max(&chars, index, digits - jolts.len());
            index = max_index + 1;
            jolts.push(max);
        }
        total_jolts += jolts.iter().collect::<String>().parse::<usize>().unwrap()
    }
    total_jolts
}

fn find_valid_max(
    chars: &Vec<char>,
    index_to_start_from: usize,
    remaining: usize,
) -> (char, usize) {
    let mut max = '0';
    let mut max_index = 0;
    for i in index_to_start_from..chars.len() {
        if i + remaining > chars.len() {
            break;
        }
        if chars[i] > max {
            max = chars[i];
            max_index = i;
        }
    }
    (max, max_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "987654321111111
811111111111119
234234234234278
818181911112111"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(357, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3121910778619, part2(&get_input()));
    }
}
