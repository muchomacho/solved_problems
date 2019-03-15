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

static INF: i64 = 100_000_000;
fn main() {
    let n = read::<usize>();
    let x = read_vector::<i64>();
    let y = read_vector::<i64>();
    let r = read_vector::<i64>();
    let s = read_vector::<i64>();

    let mut adj = AdjacencyList::new(n + 2);
    let (src, snk) = (n, n + 1);
    let mut ans = 0;
    for i in 0..n {
        if s[i] > 0 {
            ans += s[i];
            adj.add_edge(i, snk, s[i], 0, 0);
        } else {
            adj.add_edge(src, i, -s[i], 0, 0);
        }
    }
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if (x[i] - x[j]).pow(2) + (y[i] - y[j]).pow(2) <= r[j].pow(2) {
                adj.add_edge(i, j, INF, 0, 0);
            }
            if (x[j] - x[i]).pow(2) + (y[j] - y[i]).pow(2) <= r[i].pow(2) {
                adj.add_edge(j, i, INF, 0, 0);
            }
        }
    }

    ans -= maximal_flow(src, snk, &mut adj);
    println!("{}", ans);
}

type Cost = i64;
type Capacity = i64;
type Vertex = usize;
type Index = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
    rev: usize,
}

#[derive(Clone, Debug)]
struct AdjacencyList {
    len: usize,
    lists: Vec<Vec<Edge>>,
}

impl AdjacencyList {
    fn new(len: usize) -> AdjacencyList {
        AdjacencyList { len, lists: vec![vec![]; len] }
    }

    fn add_edge(&mut self, from: Vertex, to: Vertex, cap: Capacity, cost: Cost, rev_cap: Capacity) {
        let (from_dim, to_dim) = (self.lists[from].len(), self.lists[to].len());
        self.lists[from].push(Edge {
            to,
            cap,
            cost,
            rev: to_dim,
        });
        self.lists[to].push(Edge {
            to: from,
            cap: rev_cap,
            cost: -cost,
            rev: from_dim,
        });
    }

    fn len(&self) -> usize {
        self.len
    }
}


fn maximal_flow(src: Vertex, snk: Vertex, adj: &mut AdjacencyList) -> Capacity {
    let mut flow = 0;
    while let Some(path) = dfs(src, snk, &adj) {
        let min_cap = path.iter().fold(std::i64::MAX, |acc, &(v, i)| if adj.lists[v][i].cap < acc { adj.lists[v][i].cap } else { acc });
        for &(v, i) in path.iter() {
            adj.lists[v][i].cap -= min_cap;
            let (to, rev) = (adj.lists[v][i].to, adj.lists[v][i].rev);
            adj.lists[to][rev].cap += min_cap;
        }
        flow += min_cap;
    }
    flow
}

fn dfs(src: usize, snk: usize, adj: &AdjacencyList) -> Option<Vec<(Vertex, Index)>> {
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
        for (i, ref e) in adj.lists[crt].iter().enumerate() {
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
