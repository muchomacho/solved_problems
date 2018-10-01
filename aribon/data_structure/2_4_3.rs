#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap, HashMap};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write, BufReader, BufWriter};
#[allow(unused_imports)]
use std::io::prelude::BufRead;


fn main() {
    let n = read::<usize>();
    let mut chain = UnionFindTree::new(3 * n);
    let m = read::<usize>();

    let mut answer = 0;
    for _ in 0..m {
        let v = read_vector::<usize>();
        let (t, a, b) = (v[0], v[1] - 1, v[2] - 1);
        if a >= n || b >= n {
            answer += 1;
            continue;
        }
        if t == 1 {
            if chain.is_same_group(a, n + b) || chain.is_same_group(a, 2 * n + b) {
                answer += 1;
                continue;
            }
            chain.union(a, b);
            chain.union(n + a, n + b);
            chain.union(2 * n + a, 2 * n + b);
        } else if t == 2{
            if chain.is_same_group(a, b) || chain.is_same_group(a, 2 * n + b) {
                answer += 1;
                continue;
            }
            chain.union(a, n + b);
            chain.union(n + a, 2 * n + b);
            chain.union(2 * n + a, b);
        }
    }

    println!("{}", answer);

}

struct UnionFindTree {
    parents: Vec<usize>,
    heights: Vec<usize>
}

impl UnionFindTree {
    fn new(size: usize) -> UnionFindTree {
        UnionFindTree{parents: (0..size).collect(), heights: vec![0; size]}
    }

    fn climb_up(&mut self, node: usize) -> usize {
        let mut parents_to_root = vec![node];
        let mut present = node;

        loop {
            if self.parents[present] == present {
                for node in parents_to_root.iter() {
                    self.parents[*node] = present;
                }
                return present;
            }
            present = self.parents[present];
            parents_to_root.push(present);
        }
    }

    fn is_same_group(&mut self, x: usize, y: usize) -> bool {
        self.climb_up(x) == self.climb_up(y)
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.climb_up(x);
        let root_y = self.climb_up(y);

        if self.heights[root_x] > self.heights[root_y] {
            self.parents[root_y] = root_x;
        } else if self.heights[root_x] < self.heights[root_y] {
            self.parents[root_x] = root_y
        } else {
            self.parents[root_y] = root_x;
            self.heights[root_x] += 1;
        }
    }
}

#[allow(dead_code)]
fn read<T>() -> T where T: std::str::FromStr, T::Err: std::fmt::Debug {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return buf
        .trim()
        .parse()
        .unwrap();
}

#[allow(dead_code)]
fn read_vector<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    let mut buf = String::with_capacity(100);
    stdin().read_line(&mut buf).unwrap();
    return buf
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}

#[allow(dead_code)]
fn read_matrix<T>() -> Vec<Vec<T>> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    use std::io::prelude::*;
    let stdin = stdin();
    let mut reader = BufReader::with_capacity(100 * 1024, stdin);
    let mut line = String::with_capacity(100);
    let mut matrix: Vec<Vec<T>> = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        matrix.push(line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect());
        line.clear();
    }

    return matrix;
}