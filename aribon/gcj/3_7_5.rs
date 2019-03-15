#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::prelude::BufRead;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    let line = read_vector::<u64>();
    let (a, b, p) = (line[0], line[1], line[2]);
    if b < 4 {
        println!("{}", b - a + 1);
        return;
    }
    let mut arr = (a..(b + 1)).collect::<Vec<_>>();
    let mut union_find = UnionFindTree::new(arr.len());
    let b_sqrt = (b as f64).sqrt().floor() as usize;
    let mut ert = vec![true; b_sqrt + 1];
    for i in 2..(b_sqrt + 1) {
        if ert[i] {
            let mut index = i;
            while index < ert.len() {
                ert[index] = false;
                index += i;
            }
            let divider = i as u64;
            let mut index = 0;
            while arr[index] % divider != 0 && index < arr.len() {
                index += 1;
            }
            if index == arr.len() {
                continue;
            }
            let first = index;
            while index < arr.len() {
                while arr[index] % divider == 0 {
                    arr[index] /= divider;
                }
                if divider >= p {
                    union_find.unite(index, first);
                }
                index += i;
            }
        }
    }
    for i in 0..arr.len() {
        if arr[i] > 1 && arr[i] >= p && arr[i] < (arr.len() - i) as u64 {
            let (first, mut index) = (i, i + arr[i] as usize);
            while index < arr.len() {
                arr[index] = 1;
                union_find.unite(index, first);
                index += arr[i] as usize;
            }
        }
    }

    println!("{}", union_find.group_num());
}

struct UnionFindTree {
    parents: Vec<usize>,
    heights: Vec<usize>,
    sizes: Vec<usize>,
    group_num: usize,
}

impl UnionFindTree {
    fn new(size: usize) -> UnionFindTree {
        UnionFindTree {
            parents: (0..size).collect(),
            heights: vec![0; size],
            sizes: vec![1; size],
            group_num: size,
        }
    }

    fn climb_up(&mut self, node: usize) -> usize {
        let mut parents_to_root = vec![];
        let mut present = node;
        while self.parents[present] != present {
            parents_to_root.push(present);
            present = self.parents[present];
        }
        for &node in parents_to_root.iter() {
            self.parents[node] = present;
        }
        present
    }

    fn is_same_group(&mut self, x: usize, y: usize) -> bool {
        self.climb_up(x) == self.climb_up(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.climb_up(x);
        let root_y = self.climb_up(y);
        if root_x == root_y {
            return;
        }
        if self.heights[root_x] > self.heights[root_y] {
            self.parents[root_y] = root_x;
            self.sizes[root_x] += self.sizes[root_y];
        } else if self.heights[root_x] < self.heights[root_y] {
            self.parents[root_x] = root_y;
            self.sizes[root_y] += self.sizes[root_x];
        } else {
            self.parents[root_y] = root_x;
            self.heights[root_x] += 1;
            self.sizes[root_x] += self.sizes[root_y];
        }
        self.group_num -= 1;
    }

    fn group_num(&self) -> usize {
        self.group_num
    }
}

#[allow(dead_code)]
fn read<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return buf.trim().parse().unwrap();
}

#[allow(dead_code)]
fn read_vector<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::with_capacity(100);
    stdin().read_line(&mut buf).unwrap();
    return buf.split_whitespace().map(|s| s.parse().unwrap()).collect();
}

#[allow(dead_code)]
fn read_matrix<T>() -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    use std::io::prelude::*;
    let stdin = stdin();
    let mut reader = BufReader::with_capacity(100 * 1024, stdin);
    let mut line = String::with_capacity(100);
    let mut matrix: Vec<Vec<T>> = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        matrix.push(
            line.trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
        line.clear();
    }

    return matrix;
}

#[allow(dead_code)]
fn read_chars() -> Vec<char> {
    let stdin = stdin();
    let mut buf = String::new();
    let _bytes = stdin.read_line(&mut buf).unwrap();
    return buf.trim().chars().collect();
}
