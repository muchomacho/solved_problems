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

static OFFSET: i64 = 10000;

fn main() {
    let n = read::<usize>();
    let mut adjacency_list = vec![vec![]; n];
    let k = read::<usize>();
    for _ in 0..k {
        let param = read_vector::<usize>();
        let (a, b, c) = (param[0], param[1], param[2] as i64);
        adjacency_list[a].push((b, c));
        adjacency_list[b].push((a, c));
    }

    println!("{}", dijkstra_2(0, n - 1, n, &adjacency_list));

}

fn encode(node: usize, cost: i64) -> i64 {
    return (cost * OFFSET + node as i64) * (-1)
}

fn decode(key: i64) -> (usize, i64) {
    let node = ((key * (-1)) % OFFSET) as usize;
    let cost = (key * (-1)) / OFFSET;
    return (node, cost)
}

fn dijkstra_2(s: usize, e: usize, n: usize, adjacency_list: &Vec<Vec<(usize, i64)>>) -> i64 {
    let mut used = vec![0; n];
    let mut heap = BinaryHeap::<i64>::new();
    heap.push(encode(s, 0));
    let mut answer = 0;
    while heap.len() > 0 {
        let (node, cost) = decode(heap.pop().unwrap());
        if used[node] == 2 {
            continue;
        }
        used[node] += 1;
        if node == e && used[node] == 2 {
            answer = cost;
            break;
        }
        for &(neighbor, c) in adjacency_list[node].iter() {
            heap.push(encode(neighbor, cost + c));
        }
    }
    return answer;
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