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
    let nl = read::<usize>();
    let nd = read::<usize>();
    let mut edges = vec![];

    for i in 0..(n - 1) {
        edges.push((i + 1, i, 0));
    }
    
    for _ in 0..nl {
        let vec = read_vector::<usize>();
        let (a, b, d) = (vec[0], vec[1], vec[2] as i64);
        edges.push((a - 1, b - 1, d));
    }

    for _ in 0..nd {
        let vec = read_vector::<usize>();
        let (a, b, d) = (vec[0], vec[1], vec[2] as i64);
        edges.push((b - 1, a - 1, d * (-1)));
    }

    println!("{}", bellman_ford(n, 0, n - 1, &edges));    
}

fn bellman_ford(n: usize, s: usize, g: usize, edges: &Vec<(usize, usize, i64)>) -> i64 {
    let mut dist = vec![std::i64::MAX; n];
    dist[s] = 0;

    let mut count = 0;
    loop {
        let mut updated = false;
        for &(from, to, cost) in edges.iter() {
            if dist[from] < std::i64::MAX && dist[from] + cost < dist[to] {
                dist[to] = dist[from] + cost;
                updated = true;
            }
        }
        if !updated {
            break;
        }
        count += 1;
        if count == n {
            break;
        }
    }

    if count == n {
        -1
    } else if dist[g] == std::i64::MAX {
        -2
    } else {
        dist[g]
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

#[allow(dead_code)]
fn read_chars() -> Vec<char> {
    let stdin = stdin();
    let mut buf = String::new();
    let _bytes = stdin.read_line(&mut buf).unwrap();
    return buf.chars().collect();
}