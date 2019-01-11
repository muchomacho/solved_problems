#[allow(unused_imports)]
use std::cmp::{max, min};
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
    let _n = read::<usize>();
    let m = read::<usize>();
    let a = read_vector::<i32>();
    let mut segment_tree = SegTree::new(&a);

    for _ in 0..m {
        let query = read_vector::<usize>();
        if query.len() == 2 {
            println!("{}", segment_tree.query(query[0], query[1]));
        } else if query.len() == 3 {
            segment_tree.update(query[0], query[1], query[2] as i32);
        }
    }
}

struct SegTree {
    len: usize,
    size: usize,
    range: Vec<(i32, i32)>,
}

impl SegTree {
    fn new(values: &[i32]) -> SegTree {
        let len = values.len();
        assert!(len > 0);
        let size = 2usize.pow((len as f64).log2().ceil() as u32);
        let mut range = vec![(0, 0); 2 * size - 1];
        for i in 0..len {
            range[size + i - 1].0 = values[i];
        }
        for i in (0..(size - 1)).rev() {
            range[i].0 = range[2 * i + 1].0 + range[2 * i + 2].0;
        }
        SegTree { len, size, range }
    }

    fn update(&mut self, start: usize, end: usize, x: i32) {
        assert!(start <= end && end < self.len);
        let size = self.size;
        self.update_recursive(start, end, x, 0, 0, size - 1);
    }

    fn update_recursive(
        &mut self,
        start: usize,
        end: usize,
        x: i32,
        index: usize,
        left: usize,
        right: usize,
    ) {
        self.range[index].0 += x * (end - start + 1) as i32;
        let mid = (left + right) / 2;
        if start == left && end == right {
            self.range[index].1 += x;
        } else if end <= mid {
            self.update_recursive(start, end, x, 2 * index + 1, left, mid);
        } else if start > mid {
            self.update_recursive(start, end, x, 2 * index + 2, mid + 1, right);
        } else {
            self.update_recursive(start, mid, x, 2 * index + 1, left, mid);
            self.update_recursive(mid + 1, end, x, 2 * index + 2, mid + 1, right);
        }
    }

    fn query(&self, start: usize, end: usize) -> i32 {
        assert!(start <= end && end < self.len);
        self.query_recursive(start, end, 0, 0, self.size - 1)
    }

    fn query_recursive(
        &self,
        start: usize,
        end: usize,
        index: usize,
        left: usize,
        right: usize,
    ) -> i32 {
        let mid = (left + right) / 2;
        if start == left && end == right {
            self.range[index].0
        } else if end <= mid {
            self.range[index].1 * (end - start + 1) as i32
                + self.query_recursive(start, end, 2 * index + 1, left, mid)
        } else if start > mid {
            self.range[index].1 * (end - start + 1) as i32
                + self.query_recursive(start, end, 2 * index + 2, mid + 1, right)
        } else {
            self.range[index].1 * (end - start + 1) as i32
                + self.query_recursive(start, mid, 2 * index + 1, left, mid)
                + self.query_recursive(mid + 1, end, 2 * index + 2, mid + 1, right)
        }
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
