use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let (start, splitters, limits) = parse(input);
    count_splits(start, &splitters, limits)
}

pub fn part2(input: &[String]) -> usize {
    let (start, splitters, limits) = parse(input);
    count_universes(start, &splitters, limits, &mut HashMap::new())
}

fn count_splits(
    start: (usize, usize),
    splitters: &HashSet<(usize, usize)>,
    limits: (usize, usize),
) -> usize {
    let mut active_beams = HashSet::new();
    active_beams.insert(start);
    let mut splits = 0;
    while active_beams.len() > 0 {
        let mut new_active_beams: HashSet<(usize, usize)> = HashSet::new();
        for (r, c) in &active_beams {
            if splitters.contains(&(*r, *c)) {
                // println!("{:?}", (*r, *c));
                splits += 1;
                if is_valid((r + 1, c + 1), limits) {
                    new_active_beams.insert((r + 1, c + 1));
                }
                if *c > 0 && is_valid((r + 1, c - 1), limits) {
                    new_active_beams.insert((r + 1, c - 1));
                }
            } else {
                if is_valid((r + 1, *c), limits) {
                    new_active_beams.insert((r + 1, *c));
                }
            }
        }
        // for i in 0..limits.0 {
        //     for j in 0..limits.1 {
        //         if active_beams.contains(&(i, j)) {
        //             print!("|")
        //         } else if splitters.contains(&(i, j)) {
        //             print!("^")
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!()
        // }
        // println!();
        active_beams = new_active_beams;
    }
    splits
}

fn count_universes(
    (r, c): (usize, usize),
    splitters: &HashSet<(usize, usize)>,
    limits: (usize, usize),
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if !is_valid((r, c), limits) {
        return 1;
    }
    if let Some(&cached) = memo.get(&(r, c)) {
        return cached;
    }
    let mut universes = 0;
    if splitters.contains(&(r, c)) {
        universes += count_universes((r + 1, c + 1), splitters, limits, memo);
        universes += count_universes((r + 1, c.saturating_sub(1)), splitters, limits, memo);
    } else {
        universes += count_universes((r + 1, c), splitters, limits, memo);
    }
    memo.insert((r, c), universes);
    universes
}

fn is_valid(pos: (usize, usize), limit: (usize, usize)) -> bool {
    if pos.0 > limit.0 || pos.1 > limit.1 {
        return false;
    }
    true
}

fn parse(strings: &[String]) -> ((usize, usize), HashSet<(usize, usize)>, (usize, usize)) {
    let rows = strings.len();
    let cols = strings.first().map(|s| s.len()).unwrap_or(0);

    let mut start = None;
    let mut splitters = HashSet::with_capacity(rows.saturating_mul(cols));

    for (r, line) in strings.iter().enumerate() {
        for (c, b) in line.as_bytes().iter().enumerate() {
            match *b {
                b'S' => {
                    if start.is_none() {
                        start = Some((r, c));
                    }
                }
                b'^' => {
                    splitters.insert((r, c));
                }
                _ => {}
            }
        }
    }

    (start.expect("no S found"), splitters, (rows, cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(40, part2(&get_input()));
    }
}
