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

static G: f64 = 10.0;

fn main() {
    let n = read::<usize>();
    let h = read::<f64>();
    let r = read::<f64>();
    let t = read::<f64>();

    let mut positions: Vec<_> = (0..n).map(|i| position(t - i as f64, h)).collect();
    positions.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!(
        "{}",
        (0..n)
            .map(|i| (positions[i] + 2.0 * (r / 100.0) * i as f64).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn position(t: f64, h: f64) -> f64 {
    let period = ((2.0 * h) / G).sqrt();
    let k = (t / period) as usize;
    if k % 2 == 0 {
        h - 0.5 * G * (t - k as f64 * period).powi(2)
    } else {
        G * period * (t - k as f64 * period) - 0.5 * (t - k as f64 * period).powi(2)
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
