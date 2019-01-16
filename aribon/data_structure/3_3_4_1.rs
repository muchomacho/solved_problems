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
    let a = read_vector::<usize>();
    let segtree = SegTree::new(&a);

    for _ in 0..m {
        let q = read_vector::<usize>();
        let (s, e, k) = (q[0] - 1, q[1] - 1, q[2]);
        println!("{}", segtree.query(s, e, k));
    }
    
}

struct SegTree {
    len: usize,
    size: usize,
    range: Vec<Vec<usize>>,
    minimum: usize,
    maximum: usize,
}

impl SegTree {
    fn new(values: &[usize]) -> SegTree {
        let len = values.len();
        assert!(len > 0);
        let (mut minimum, mut maximum) = (std::usize::MAX, 0);
        for elem in values {
            if *elem < minimum {
                minimum = *elem;
            }
            if *elem > maximum {
                maximum = *elem;
            }
        }
        let size = 2usize.pow((len as f64).log2().ceil() as u32);
        let mut range = vec![vec![]; 2 * size - 1];
        for i in 0..len {
            range[size + i - 1].push(values[i]);
        }
        for i in (0..(size - 1)).rev() {
            let (l, r) = (2 * i + 1, 2 * i + 2);
            let (mut l_index, mut r_index) = (0, 0);
            while l_index < range[l].len() || r_index < range[r].len() {
                if l_index < range[l].len() && (r_index == range[r].len() || range[l][l_index] <= range[r][r_index]) {
                    let x = range[l][l_index];
                    range[i].push(x);
                    l_index += 1;
                } else {
                    let x = range[r][r_index];
                    range[i].push(x);
                    r_index += 1;
                }
            }
        }
        SegTree { len, size, range, minimum, maximum }
    }

    fn query(&self, start: usize, end: usize, k: usize) -> usize {
        assert!(start <= end && end < self.len);
        if self.query_recursive(self.minimum, start, end, 0, 0, self.size - 1) >= k {
            return self.minimum;
        }
        let (mut l, mut r) = (self.minimum, self.maximum);
        while r - l > 1 {
            let mid = (l + r) / 2;
            if self.query_recursive(mid, start, end, 0, 0, self.size - 1) < k {
                l = mid;
            } else {
                r = mid;
            }
        }
        r
    }

    fn query_recursive(
        &self,
        x: usize,
        start: usize,
        end: usize,
        index: usize,
        left: usize,
        right: usize,
    ) -> usize {
        let mid = (left + right) / 2;
        if start == left && end == right {
            match self.range[index].binary_search(&x) {
                Ok(i) => i + 1,
                Err(i) => i
            }
        } else if end <= mid {
            self.query_recursive(x, start, end, 2 * index + 1, left, mid)
        } else if start > mid {
            self.query_recursive(x, start, end, 2 * index + 2, mid + 1, right)
        } else {
            self.query_recursive(x, start, mid, 2 * index + 1, left, mid) + self.query_recursive(x, mid + 1, end, 2 * index + 2, mid + 1, right)   
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
