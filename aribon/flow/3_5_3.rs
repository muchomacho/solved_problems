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

type Point = usize;
type Cost = i32;
type Capacity = i32;

fn main() {
    let n = read::<usize>();
    let s = read::<usize>();
    let t = read::<usize>();
    let f = read::<i32>();
    let m = read::<usize>();

    let mut adj = vec![HashMap::new(); n];
    for _ in 0..m {
        let line = read_vector::<usize>();
        adj[line[0]].insert(line[1], (line[2] as i32, line[3] as i32));
    }

    println!("{}", min_cost_flow(&adj, s, t, f));
}

fn min_cost_flow(adj: &[HashMap<Point, (Cost, Capacity)>], src: Point, snk: Point, f: i32) -> i32 {
    let mut res_adj = vec![HashMap::new(); adj.len()];
    for i in 0..adj.len() {
        for (key, val) in adj[i].iter() {
            res_adj[i].insert(*key, *val);
            res_adj[*key].insert(i, (val.0 * (-1), 0));
        }
    }

    let mut flow = 0;
    let mut cost = 0;
    let mut h = vec![(0, 0); adj.len()];
    loop {
        h = dijkstra(&res_adj, &h, src);
        if h[snk].0 == std::i32::MAX {
            cost = -1;
            break;
        }
        let mut path = vec![snk, h[snk].1];
        loop {
            let prv = h[path[path.len() - 1]].1;
            path.push(prv);
            if prv == src {
                break;
            }
        }
        path.reverse();
        let min_cap = (0..path.len() - 1).fold(std::i32::MAX, |acc, i| {
            min(res_adj[path[i]].get(&path[i + 1]).unwrap().1, acc)
        });
        let new_flow = min(min_cap, f - flow);
        for i in 0..(path.len() - 1) {
            {
                let p = res_adj[path[i]].get_mut(&path[i + 1]).unwrap();
                p.1 -= new_flow;
            }
            {
                let p = res_adj[path[i + 1]].get_mut(&path[i]).unwrap();
                p.1 += new_flow;
            }
        }
        flow += new_flow;
        cost += h[snk].0 * new_flow;
        if flow == f {
            break;
        }
    }
    cost
}

fn dijkstra(
    adj: &[HashMap<Point, (Cost, Capacity)>],
    h: &[(Cost, Point)],
    src: Point,
) -> Vec<(Cost, Point)> {
    let mut new_h = vec![(std::i32::MAX, 0); adj.len()];
    let mut dist = BinaryHeap::new();
    dist.push((0, src, src));
    while dist.len() > 0 {
        let minimum = dist.pop().unwrap();
        let cost = minimum.0 * (-1);
        let (crt, prv) = (minimum.1, minimum.2);
        if new_h[crt].0 < std::i32::MAX {
            continue;
        }
        new_h[crt] = (cost + h[crt].0, prv);
        for (key, val) in adj[crt].iter() {
            if new_h[*key].0 < std::i32::MAX || val.1 == 0 {
                continue;
            }
            dist.push(((cost + val.0 + h[crt].0 - h[*key].0) * (-1), *key, crt));
        }
    }
    println!("{:?}", new_h);
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
