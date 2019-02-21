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

fn main() {
    let n = read::<usize>();
    let k = read::<usize>();
    let mut adj = vec![HashMap::new(); n * 2 + 2];
    for _ in 0..k {
        let line = read_vector::<usize>();
        let (x, y) = (line[0] - 1, line[1] - 1);
        adj[x].insert(n + y, 1);
    }

    for i in 0..n {
        adj[n * 2].insert(i, 1);
        adj[n + i].insert(n * 2 + 1, 1);
    }
    
    println!("{}", max_flow(n * 2, n * 2 + 1, &adj));
}

fn max_flow(src: usize, snk: usize, adj: &[HashMap<usize, usize>]) -> usize {
    let mut res_adj = vec![HashMap::new(); adj.len()];
    for i in 0..adj.len() {
        for (&key, &val) in adj[i].iter() {
            res_adj[i].insert(key, val);
            res_adj[key].insert(i, 0);
        }
    }
    let mut adj = res_adj;

    let mut flow = 0;
    while let Some(path) = bfs(src, snk, &adj) {
        let min_cap = (0..(path.len() - 1)).fold(std::usize::MAX, |acc, i| min(acc, *adj[path[i]].get(&path[i + 1]).unwrap()));
        flow += min_cap;
        for i in 0..(path.len() - 1) {
            *adj[path[i]].get_mut(&path[i + 1]).unwrap() -= min_cap;
            *adj[path[i + 1]].get_mut(&path[i]).unwrap() += min_cap;
        }
    }

    flow
}

fn bfs(src: usize, snk: usize, adj: &[HashMap<usize, usize>]) -> Option<Vec<usize>> {
    let mut used = vec![false; adj.len()];
    let mut queue = VecDeque::new();
    let mut stack = VecDeque::new();
    queue.push_back((src, src));
    while queue.len() > 0 {
        let p = queue.pop_front().unwrap();
        let crt = p.0;
        stack.push_back(p);
        if crt == snk {
            break;
        }
        for (&nxt, &cap) in adj[crt].iter() {
            if used[nxt] || cap == 0 {
                continue;
            }
            queue.push_back((nxt, crt));
            used[nxt] = true;
        }
    }

    let mut tb = stack.pop_back().unwrap();
    if tb.0 != snk {
        return None
    }
    let mut path = vec![tb.0, tb.1];
    while tb.1 != src {
        let new_tb = stack.pop_back().unwrap();
        if new_tb.0 == tb.1 {
            path.push(new_tb.1);
            tb = new_tb;
        }
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
