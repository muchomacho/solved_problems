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

fn main() {
    let n = read::<usize>();
    let m = read::<usize>();

    let mut adj = vec![vec![]; (n + 2) * (m + 2) + 2];
    let (src, snk) = ((n + 2) * (m + 2), (n + 2) * (m + 2) + 1);
    for i in 1..(n + 1) {
        let line = read_chars();
        for j in 1..(m + 1) {
            if line[j - 1] == 'x' {
                continue;
            }
            if (i + j) % 2 == 0 {
                add_edge(src, i * m + j, 1, 0, 0, &mut adj);
                add_edge(i * m + j, (i - 1) * m + j, 1, 0, 0, &mut adj);
                add_edge(i * m + j, (i + 1) * m + j, 1, 0, 0, &mut adj);
                add_edge(i * m + j, i * m + j - 1, 1, 0, 0, &mut adj);
                add_edge(i * m + j, i * m + j + 1, 1, 0, 0, &mut adj);
            } else {
                add_edge(i * m + j, snk, 1, 0, 0, &mut adj);
                add_edge((i - 1) * m + j, i * m + j, 1, 0, 0, &mut adj);
                add_edge((i + 1) * m + j, i * m + j, 1, 0, 0, &mut adj);
                add_edge(i * m + j - 1, i * m + j, 1, 0, 0, &mut adj);
                add_edge(i * m + j + 1, i * m + j, 1, 0, 0, &mut adj);
            }
        }
    }

    println!("{}", (n * m) as i64 - maximal_flow(src, snk, &mut adj));
}

type Cost = i64;
type Vertex = usize;
type Index = usize;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
    rev: usize,
}

fn add_edge(from: Vertex, to: Vertex, cap: i64, cost: Cost, rev_cap: i64, adj: &mut Vec<Vec<Edge>>) {
    let (from_dim, to_dim) = (adj[from].len(), adj[to].len());
    adj[from].push(Edge {
        to,
        cap,
        cost,
        rev: to_dim,
    });
    adj[to].push(Edge {
        to: from,
        cap: rev_cap,
        cost: -cost,
        rev: from_dim,
    });
}

fn maximal_flow(src: usize, snk: usize, adj: &mut Vec<Vec<Edge>>) -> i64 {
    let mut flow = 0;
    while let Some(path) = dfs(src, snk, &adj) {
        let min_cap = path.iter().fold(std::i64::MAX, |acc, &(v, i)| if adj[v][i].cap < acc { adj[v][i].cap } else { acc });
        for &(v, i) in path.iter() {
            adj[v][i].cap -= min_cap;
            let (to, rev) = (adj[v][i].to, adj[v][i].rev);
            adj[to][rev].cap += min_cap;
        }
        flow += min_cap;
    }
    flow
}

fn dfs(src: usize, snk: usize, adj: &Vec<Vec<Edge>>) -> Option<Vec<(Vertex, Index)>> {
    let mut used = vec![false; adj.len()];
    let mut stack = VecDeque::new();
    let mut used_stack = VecDeque::new();
    stack.push_front((src, src, 0));
    while stack.len() > 0 {
        let tuple = stack.pop_front().unwrap();
        let (crt, _prv, _index) = (tuple.0, tuple.1, tuple.2);
        used_stack.push_front(tuple);
        if crt == snk {
            break;
        }
        for (i, ref e) in adj[crt].iter().enumerate() {
            if used[e.to] || e.cap == 0 {
                continue;
            }
            stack.push_front((e.to, crt, i));
            used[e.to] = true;
        }
    }

    let mut tb = used_stack.pop_front().unwrap();
    if tb.0 != snk {
        return None;
    }
    let mut path = vec![(tb.1, tb.2)];
    while tb.1 != src {
        let new_tb = used_stack.pop_front().unwrap();
        if new_tb.0 != tb.1 {
            continue;
        }
        tb = new_tb;
        path.push((tb.1, tb.2));
    }
    path.reverse();
    Some(path)
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
