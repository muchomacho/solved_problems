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
    let m = read::<usize>();
    let mut adj = vec![HashMap::new(); n + 2];
    let (src, snk) = (n, n + 1);
    for i in 0..n {
        let line = read_vector::<usize>();
        adj[src].insert(i, line[0]);
        adj[i].insert(src, 0);
        adj[i].insert(snk, line[1]);
        adj[snk].insert(i, 0);
    }
    for _ in 0..m {
        let line = read_vector::<usize>();
        let (a, b, cost) = (line[0] - 1, line[1] - 1, line[2]);
        adj[a].insert(b, cost);
        adj[b].insert(a, cost);
    }

    println!("{}", max_flow(src, snk, &mut adj));

}

fn max_flow(src: usize, snk: usize, adj: &mut [HashMap<usize, usize>]) -> usize {
    let mut flow = 0;
    while let Some(path) = dfs(src, snk, &adj) {
        for i in 0..(path.len() - 1) {
            *adj[path[i]].get_mut(&path[i + 1]).unwrap() -= 1;
            *adj[path[i + 1]].get_mut(&path[i]).unwrap() += 1;
        }
        flow += 1;
    }
    flow
}

fn dfs(src: usize, snk: usize, adj: &[HashMap<usize, usize>]) -> Option<Vec<usize>> {
    let mut used = vec![false; adj.len()];
    let mut stack = VecDeque::new();
    let mut used_stack = VecDeque::new();
    stack.push_front((src, src));
    while stack.len() > 0 {
        let p = stack.pop_front().unwrap();
        used_stack.push_front(p);
        let crt = p.0;
        if crt == snk {
            break;
        }
        for (&nxt, &cap) in adj[crt].iter() {
            if used[nxt] || cap == 0 {
                continue;
            }
            stack.push_front((nxt, crt));
            used[nxt] = true;
        }
    }

    let mut tb = used_stack.pop_front().unwrap();
    if tb.0 != snk {
        return None
    }
    let mut path = vec![tb.0, tb.1];
    while tb.1 != src {
        let new_tb = used_stack.pop_front().unwrap();
        if new_tb.0 == tb.1 {
            tb = new_tb;
            path.push(tb.1);
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
