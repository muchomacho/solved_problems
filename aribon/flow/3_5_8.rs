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

type Cost = i64;
type Vertex = usize;
type Index = usize;

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
    rev: usize,
}

fn main() {
    let n = read::<usize>();
    let m = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let line = read_vector::<usize>();
        let (a, b, cost) = (line[0] - 1, line[1] - 1, line[2] as i64);
        add_edge(a, b, 1, cost, &mut adj);
        add_edge(b, a, 1, cost, &mut adj);
    }

    println!("{}", minimal_cost_flow(0, n - 1, 2, &mut adj).unwrap());
}

fn add_edge(from: usize, to: usize, cap: i64, cost: i64, adj: &mut Vec<Vec<Edge>>) {
    let (from_dim, to_dim) = (adj[from].len(), adj[to].len());
    adj[from].push(Edge {
        to,
        cap,
        cost,
        rev: to_dim,
    });
    adj[to].push(Edge {
        to: from,
        cap: 0,
        cost: -cost,
        rev: from_dim,
    });
}

fn minimal_cost_flow(src: usize, snk: usize, flow: i64, adj: &mut Vec<Vec<Edge>>) -> Option<i64> {
    let mut total_flow = 0;
    let mut cost = 0;
    let mut h = vec![(0, std::usize::MAX, std::usize::MAX); adj.len()];
    while total_flow < flow {
        h = dijkstra(src, &h, &adj);
        let c = h[snk].0;
        if c == std::i64::MAX {
            return None;
        }
        let mut crt = snk;
        let mut min_cap = std::i64::MAX;
        while crt != src {
            let (prv, index) = (h[crt].1, h[crt].2);
            min_cap = min(min_cap, adj[prv][index].cap);
            crt = prv;
        }
        let inc = min(min_cap, flow - total_flow);
        total_flow += inc;
        cost += inc * c;
    }

    Some(cost)
}

fn dijkstra(
    src: usize,
    h: &[(Cost, Vertex, Index)],
    adj: &Vec<Vec<Edge>>,
) -> Vec<(Cost, Vertex, Index)> {
    let mut new_h = vec![(std::i64::MAX, std::usize::MAX, std::usize::MAX); adj.len()];
    let mut used = vec![false; adj.len()];
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((0, src, std::usize::MAX, std::usize::MAX));
    used[src] = true;
    while priority_queue.len() > 0 {
        let p = priority_queue.pop().unwrap();
        let (cost, crt, prv, index) = (p.0 * (-1), p.1, p.2, p.3);
        if new_h[crt].0 < std::i64::MAX {
            continue;
        }
        new_h[crt] = (cost + h[crt].0, prv, index);
        for (i, e) in adj[crt].iter().enumerate() {
            if new_h[e.to].0 < std::i64::MAX || e.cap == 0 {
                continue;
            }
            priority_queue.push(((cost + e.cost + h[crt].0 - h[e.to].0) * (-1), e.to, crt, i));
        }
    }
    new_h
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
