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
    let n = read::<usize>();
    let c = read::<usize>();
    let l = read_vector::<usize>();
    let s = read_vector::<usize>().iter().map(|i| *i - 1).collect::<Vec<_>>();
    let a = read_vector::<f64>();

    let mut previous_angles = vec![std::f64::consts::PI; n];
    let mut segment_tree = SegTree::new(&l);
    for i in 0..c {
        let angle = a[i] * std::f64::consts::PI / 180.0 - previous_angles[s[i]];
        segment_tree.update(s[i], angle);
        let vec = segment_tree.range[0].0;
        println!("{:.2} {:.2}", vec.0, vec.1);
        previous_angles[s[i]] = a[i];
    }
    
}

struct SegTree {
    len: usize,
    size: usize,
    range: Vec<((f64, f64), f64)>,
}

impl SegTree {
    fn new(values: &[usize]) -> SegTree {
        let len = values.len();
        assert!(len > 0);
        let size = 2usize.pow((len as f64).log2().ceil() as u32);
        let mut range = vec![((0.0, 0.0), 0.0); 2 * size - 1];
        for i in 0..len {
            range[size + i - 1].0 = (0.0, values[i] as f64);
        }
        for i in (0..(size - 1)).rev() {
            (range[i].0).1 = (range[2 * i + 1].0).1 + (range[2 * i + 2].0).1;
        }
        SegTree { len, size, range }
    }

    fn update(&mut self, i: usize, x: f64) {
        assert!(i < self.len);
        let size = self.size;
        self.update_recursive(i, x, 0, 0, size - 1);
    }

    fn update_recursive(&mut self, i: usize, x: f64, index: usize, left: usize, right: usize) {
        if i < left || i >= right || left == right {
            return;
        }

        let mid = (left + right) / 2;
        if i <= mid {
            self.range[index].1 += x;
        }

        self.update_recursive(i, x, 2 * index + 1, left, mid);
        self.update_recursive(i, x, 2 * index + 2, mid + 1, right);

        let (cos, sin) = ((self.range[index].1).cos(), (self.range[index].1).sin());
        self.range[index].0 = (
            (self.range[2 * index + 1].0).0 
                + (self.range[2 * index + 2].0).0 * cos
                - (self.range[2 * index + 2].0).1 * sin,
            (self.range[2 * index + 1].0).1
                + (self.range[2 * index + 2].0).0 * sin
                + (self.range[2 * index + 2].0).1 * cos,
        );
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
