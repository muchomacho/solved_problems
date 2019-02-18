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

static MAX: i64 = 1_000_000_000_000_000_000;

fn main() {
    let line = read_vector::<i64>();
    let (x1, y1) = (line[0], line[1]);
    let line = read_vector::<i64>();
    let (x2, y2) = (line[0], line[1]);
    let n = read::<usize>();
    let s = read_chars();

    let dest = (x2 - x1, y2 - y1);

    let mut cumsum = vec![];
    let mut prv = (0, 0);
    for i in 0..n {
        let crt = match s[i] {
            'U' => (prv.0, prv.1 + 1),
            'D' => (prv.0, prv.1 - 1),
            'L' => (prv.0 - 1, prv.1),
            'R' => (prv.0 + 1, prv.1),
            _ => panic!("Invalid character"),
        };
        cumsum.push(crt);
        prv = crt;
    }

    if dist(MAX, &cumsum, dest) > MAX {
        println!("-1");
        return;
    }
    
    let (mut l, mut r) = (0, MAX);
    while r - l > 1 {
        let mid = (l + r) / 2;
        if dist(mid, &cumsum, dest) > mid {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", r);

}

fn dist(x: i64, cumsum: &[(i64, i64)], dest: (i64, i64)) -> i64 {
    let quotient = (x - 1) / cumsum.len() as i64;
    let residual = ((x - 1) % cumsum.len() as i64) as usize;
    let init = (
        cumsum[cumsum.len() - 1].0 * quotient + cumsum[residual].0,
        cumsum[cumsum.len() - 1].1 * quotient + cumsum[residual].1,
    );

    (dest.0 - init.0).abs() + (dest.1 - init.1).abs()
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
