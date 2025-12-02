pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    find_invalid_ids(parsed)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    find_invalid_ids_2(parsed)
}

fn find_invalid_ids(input: Vec<(usize, usize)>) -> usize {
    let mut total = 0;
    for (start, end) in input {
        for i in start..=end {
            let i_str = i.to_string();
            if i_str.len() % 2 == 1 {
                // can't be two identical parts
                continue;
            }
            let (first_half, second_half) = i_str.split_at(i_str.len() / 2);

            if first_half == second_half {
                total += i;
            }
        }
    }
    total
}

fn find_invalid_ids_2(input: Vec<(usize, usize)>) -> usize {
    let mut total = 0;
    for (start, end) in input {
        for i in start..=end {
            let i_str = i.to_string();
            for j in 1..=i_str.len() / 2 {
                if i_str.len() % j > 0 {
                    // skip if string can't be divided evenly
                    continue;
                } 
                let subset = &i_str[..j];
                let matches = i_str.matches(subset).count();
                if i_str.len() == subset.len() * matches {
                    total += i;
                    break;
                }
            }
        }
    }
    total
}

fn parse(strings: &[String]) -> Vec<(usize, usize)> {
    let mut ranges = vec![];
    let line = strings.get(0).unwrap();
    let parts = line.split(',');
    for elem in parts {
        let mut values = elem.split('-');
        let start = values.next().unwrap().parse().unwrap();
        let end = values.next().unwrap().parse().unwrap();
        ranges.push((start, end));
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".lines().map(|s| String::from(s.trim())).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(1227775554, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4174379265, part2(&get_input()));
    }
}
