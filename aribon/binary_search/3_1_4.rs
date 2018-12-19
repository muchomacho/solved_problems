#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::prelude::BufRead;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::str::FromStr;

static MAX: f64 = 1_000_000.0;

fn main() {
    let _n = read::<i64>();
    let k = read::<usize>();
    let w = read_vector::<i64>();
    let v = read_vector::<i64>();

    if judge(MAX, k, &w, &v) {
        println!("{}", MAX);
        return;
    }

    let mut left = 0.0;
    let mut right = MAX;
    for _ in 0..100 {
        let mid = (left + right) / 2.0;
        if judge(mid, k, &w, &v) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}

fn judge(x: f64, k: usize, w: &[i64], v: &[i64]) -> bool {
    let mut compare: Vec<_> = (0..w.len()).map(|i| v[i] as f64 - x * w[i] as f64).collect();
    compare.sort_by(|a, b| a.partial_cmp(b).unwrap());
    compare.reverse();

    compare[..k].iter().fold(0.0, |acc, i| acc + *i) >= 0.0
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
    return buf.chars().collect();
}
