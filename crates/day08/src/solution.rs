use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn part1(input: &[String]) -> usize {
    let junction_boxes = parse(input);
    product_after_k_edges(&junction_boxes, 1000)
}

pub fn part2(input: &[String]) -> usize {
    let junction_boxes = parse(input);
    x_product_on_full_connect(&junction_boxes) as usize
}

fn product_after_k_edges(junction_boxes: &[JunctionBox], k: usize) -> usize {
    let n = junction_boxes.len();
    let mut heap = build_edge_heap(junction_boxes);

    let mut dsu = DSU::new(n);
    let mut processed = 0usize;

    while processed < k {
        let e = heap.pop().unwrap();
        processed += 1;
        dsu.union(e.i, e.j);
    }

    product_of_top3_component_sizes(&mut dsu, n)
}

fn product_of_top3_component_sizes(dsu: &mut DSU, n: usize) -> usize {
    let mut sizes = vec![0usize; n];
    for v in 0..n {
        let r = dsu.find(v);
        sizes[r] += 1;
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

fn x_product_on_full_connect(junction_boxes: &[JunctionBox]) -> i64 {
    let n = junction_boxes.len();
    let mut heap = build_edge_heap(junction_boxes);

    let mut dsu = DSU::new(n);
    let mut components = n;

    while let Some(e) = heap.pop() {
        // Part 2: consider only "unconnected pairs" => only edges that MERGE components
        if dsu.union(e.i, e.j) {
            components -= 1;
            if components == 1 {
                return junction_boxes[e.i].0 * junction_boxes[e.j].0;
            }
        }
    }

    unreachable!("Should become connected after enough edges");
}

fn build_edge_heap(junction_boxes: &[JunctionBox]) -> BinaryHeap<Edge> {
    let n = junction_boxes.len();
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            heap.push(Edge {
                i,
                j,
                d2: junction_boxes[i].dist2(&junction_boxes[j]),
            });
        }
    }
    heap
}

#[derive(Clone, Copy)]
struct JunctionBox(i64, i64, i64);

impl JunctionBox {
    fn dist2(&self, other: &JunctionBox) -> i128 {
        let dx = self.0 as i128 - other.0 as i128;
        let dy = self.1 as i128 - other.1 as i128;
        let dz = self.2 as i128 - other.2 as i128;
        dx * dx + dy * dy + dz * dz
    }
}

fn parse(strings: &[String]) -> Vec<JunctionBox> {
    strings
        .iter()
        .filter(|s| !s.trim().is_empty())
        .map(|line| {
            let mut it = line.split(',').map(|x| x.trim().parse::<i64>().unwrap());
            JunctionBox(it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Edge {
    i: usize,
    j: usize,
    d2: i128,
}

// BinaryHeap is max-heap => invert ordering to pop smallest distance first
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .d2
            .cmp(&self.d2)
            .then_with(|| self.i.cmp(&other.i))
            .then_with(|| self.j.cmp(&other.j))
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.d2 == other.d2
    }
}
impl Eq for Edge {}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }
    // returns true if it actually merged two components
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(40, product_after_k_edges(&parse(&get_input()), 10));
    }

    #[test]
    fn test_part2() {
        assert_eq!(25272, part2(&get_input()));
    }
}
