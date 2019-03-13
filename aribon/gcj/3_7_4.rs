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

static EPS: f64 = 0.000_000_000_1;

fn main() {
    let n = read::<usize>();
    let mut circles = vec![];
    for _ in 0..n {
        let line = read_vector::<f64>();
        circles.push((line[0], line[1], line[2]));
    }

    println!("{:.6}", binary_search(&circles));
}

fn binary_search(circles: &[(f64, f64, f64)]) -> f64 {
    let (mut l, mut r) = (0.0, 1000.0);
    for _ in 0..100 {
        let mid = (l + r) / 2.0;
        if can_cover(mid, circles) {
            r = mid;
        } else {
            l = mid;
        }
    }
    r
}

fn can_cover(x: f64, circles: &[(f64, f64, f64)]) -> bool {
    let mut cands = vec![];
    for i in 0..(circles.len() - 1) {
        for j in (i + 1)..circles.len() {
            if circles[i].2 >= x || circles[j].2 >= x {
                continue;
            }
            if circles[i].1 != circles[j].1 {
                let d = (x - circles[i].2).powi(2)
                    - ((circles[i].0 - circles[j].0).powi(2)
                        + (circles[i].1 - circles[j].1).powi(2))
                        / 4.0;
                if d < 0.0 - EPS {
                    continue;
                }
                let a =
                    1.0 + ((circles[i].0 - circles[j].0) / (circles[i].1 - circles[j].1)).powi(2);
                let k = (d / a).sqrt();
                cands.push((
                    (circles[i].0 + circles[j].0) / 2.0 + k,
                    (circles[i].1 + circles[j].1) / 2.0
                        - k * (circles[i].0 - circles[j].0) / (circles[i].1 - circles[j].1),
                    x,
                ));
                cands.push((
                    (circles[i].0 + circles[j].0) / 2.0 - k,
                    (circles[i].1 + circles[j].1) / 2.0
                        + k * (circles[i].0 - circles[j].0) / (circles[i].1 - circles[j].1),
                    x,
                ));
            } else {
                let d = (x - circles[i].2).powi(2) - (circles[i].0 - circles[j].0).powi(2) / 4.0;
                if d < 0.0 - EPS {
                    continue;
                }
                let k = d.sqrt();
                cands.push((
                    (circles[i].0 + circles[j].0) / 2.0,
                    (circles[i].1 + circles[j].1) / 2.0 + k,
                    x,
                ));
                cands.push((
                    (circles[i].0 + circles[j].0) / 2.0,
                    (circles[i].1 + circles[j].1) / 2.0 - k,
                    x,
                ));
            }
        }
    }
    for i in 0..circles.len() {
        if x <= circles[i].2 {
            continue;
        }
        cands.push((circles[i].0, circles[i].1, x));
    }

    let all = 2u64.pow(circles.len() as u32) - 1;
    if cands.len() < 2 {
        return false;
    }
    for i in 0..(cands.len() - 1) {
        let covered = cover(cands[i], circles);
        for j in (i + 1)..cands.len() {
            if covered | cover(cands[j], circles) == all {
                return true;
            }
        }
    }

    false
}

fn cover(c: (f64, f64, f64), circles: &[(f64, f64, f64)]) -> u64 {
    let mut covered = 0;
    for i in 0..circles.len() {
        let dist = ((c.0 - circles[i].0).powi(2) + (c.1 - circles[i].1).powi(2)).sqrt();
        if dist + circles[i].2 <= c.2 + EPS {
            covered |= 1 << i;
        }
    }
    covered
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
