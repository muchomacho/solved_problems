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
    let m = read::<usize>();
    let mut adjacency_list = vec![vec![]; n + m];
    let r = read::<usize>();

    for _ in 0..r {
        let vec = read_vector::<usize>();
        let (a, b, c) = (vec[0], vec[1], vec[2] as i64);
        adjacency_list[a].push((n + b, 10000 - c));
        adjacency_list[n + b].push((a, 10000 - c));
    }

    let mut ans = 0;
    let mut used = vec![false; n + m];
    for i in 0..(n + m) {
        if used[i] {
            continue;
        }
        ans += 10000 + minimum_spanning_tree(i, &adjacency_list, &mut used);
    }

    println!("{}", ans);
}

fn encode(tuple: (usize, i64)) -> (i64, usize) {
    (tuple.1 * (-1), tuple.0)
}

fn decode(tuple: (i64, usize)) -> (usize, i64) {
    (tuple.1, tuple.0 * (-1))
}

fn minimum_spanning_tree(s: usize, adjacency_list: &Vec<Vec<(usize, i64)>>, used: &mut Vec<bool>) -> i64 {
    let mut heap = BinaryHeap::<(i64, usize)>::new();
    heap.push((0, s));
    let mut tree = BTreeSet::<usize>::new();
    let mut ans = 0;

    while heap.len() > 0 {
        let (node, cost) = decode(heap.pop().unwrap());
        if !tree.insert(node) {
            continue;
        };
        used[node] = true;
        ans += cost;
        for &(neighbor, cost) in adjacency_list[node].iter() {
            heap.push(encode((neighbor, cost)));
        }
    }

    return ans;
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