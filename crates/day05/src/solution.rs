pub fn part1(input: &[String]) -> usize {
    let (ranges, ids) = parse(input);
    count_fresh_ingredients(ranges, ids)
}

pub fn part2(input: &[String]) -> usize {
    let (ranges, _ids) = parse(input);
    count_possible_fresh_ingredients(ranges)
}

fn count_fresh_ingredients(mut ranges: Vec<(usize, usize)>, ids: Vec<usize>) -> usize {
    ranges.sort();
    let mut fresh_ingredients_count = 0;
    for id in &ids {
        for (start, end) in &ranges {
            if id < start {
                break;
            }
            if id >= start && id <= end {
                fresh_ingredients_count += 1;
                break;
            }
        }
    }
    fresh_ingredients_count
}

fn count_possible_fresh_ingredients(mut ranges: Vec<(usize, usize)>) -> usize {
    ranges.sort();
    let mut changed = true;

    while changed {
        changed = false;
        let mut out = vec![];
        let mut i = 0;
        while i < ranges.len() {
            let mut current = ranges[i];
            let mut j = i + 1;

            while j < ranges.len() && current.1 >= ranges[j].0 {
                current.1 = current.1.max(ranges[j].1);
                changed = true;
                j += 1;
            }
            out.push(current);
            i = j;
        }
        ranges = out;
    }
    ranges.iter().map(|(a, b)| b - a + 1).sum()
}

fn parse(strings: &[String]) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = vec![];
    let mut ids = vec![];
    let mut is_first_part = true;
    for line in strings {
        if line.is_empty() {
            is_first_part = false;
            continue;
        }
        if is_first_part {
            let mut split_line = line.split('-');
            let range_start: usize = split_line.next().unwrap().parse().unwrap();
            let range_end = split_line.next().unwrap().parse().unwrap();
            ranges.push((range_start, range_end));
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
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
        assert_eq!(14, part2(&get_input()));
    }
}
