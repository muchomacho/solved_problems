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

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
    rev: usize,
}

fn main() {
    let n = read::<usize>();
    let m = read::<usize>();
    let mut buildings = vec![];
    let mut shelters = vec![];
    let mut plan = vec![vec![0; m]; n];
    for _ in 0..n {
        let line = read_vector::<i64>();
        buildings.push(((line[0], line[1]), line[2]));
    }
    for _ in 0..m {
        let line = read_vector::<i64>();
        shelters.push(((line[0], line[1]), line[2]));
    }
    for i in 0..n {
        plan[i] = read_vector::<i64>();
    }
    let mut adj = vec![vec![]; n + m + 2];
    let (src, snk) = (n + m, n + m + 1);
    let mut people_per_shelter = vec![0; m];
    for i in 0..n {
        add_edge(src, i, 0, 0, buildings[i].1, &mut adj);
        for j in 0..m {
            people_per_shelter[j] += plan[i][j];
            let cost = ((buildings[i].0).0 - (shelters[j].0).0).abs() + ((buildings[i].0).1 - (shelters[j].0).1).abs() + 1;
            add_edge(i, n + j, buildings[i].1 - plan[i][j], cost, plan[i][j], &mut adj);
        }
    }
    for j in 0..m {
        add_edge(n + j, snk, shelters[j].1 - people_per_shelter[j], 0, people_per_shelter[j], &mut adj);
    }
    if let Some(path) = find_negative_cycle(&adj) {
        println!("SUBOPTIMAL");
        println!("{:?}", path);
        let mut better_plan = plan.clone();
        for i in 0..(path.len() - 1) {
            if path[i] < n && path[i + 1] >= n && path[i + 1] < n + m {
                better_plan[path[i]][path[i + 1] - n] += 1;
            } else if path[i] >= n && path[i] < n + m && path[i + 1] < n {
                better_plan[path[i + 1]][path[i] - n] -= 1;
            }
        }
        for i in 0..n {
            let line = better_plan[i].iter().map(|x| x.to_string()).collect::<Vec<_>>();
            println!("{}", line.join(" "));
        }
    } else {
        println!("OPTIMAL");
    }
}

fn add_edge(from: usize, to: usize, cap: i64, cost: i64, rev_cap: i64, adj: &mut Vec<Vec<Edge>>) {
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

fn find_negative_cycle(adj: &Vec<Vec<Edge>>) -> Option<Vec<Vertex>> {
    // floyd-warshall can find negative cycle
    // found cycles may contain smaller cycles (length <= n) and the length can be longer than n
    let dist = floyd_warshall(&adj);
    for i in 0..adj.len() {
        if dist[i][i].0 >= 0 {
            continue;
        }

        // bellman-ford can recover negative cycle connected to source
        // with n times iteration, it can recover cycle whose length <= n
        // if a part of cycle whose length > n is passed as src, 
        // it can recover the smaller cycle which belongs to the original cycle
        return find_cycle(i, &adj);
    }
    None
}

fn floyd_warshall(adj: &Vec<Vec<Edge>>) -> Vec<Vec<(Cost, Vertex)>> {
    let n = adj.len();
    let mut dist = vec![vec![(std::i64::MAX, std::usize::MAX); n]; n];
    for i in 0..n {
        dist[i][i] = (0, std::usize::MAX);
    }
    for i in 0..n {
        for ref e in adj[i].iter() {
            if e.cap > 0 {
                dist[i][e.to] = (e.cost, std::usize::MAX);
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i == k || j == k {
                    continue;
                }
                if dist[i][k].0 == std::i64::MAX || dist[k][j].0 == std::i64::MAX {
                    continue;
                }
                let new_dist = dist[i][k].0 + dist[k][j].0;
                if new_dist < dist[i][j].0 {
                    dist[i][j] = (new_dist, k);
                }
            }
        }
    }
    dist
}

fn find_cycle(start: usize, adj: &Vec<Vec<Edge>>) -> Option<Vec<Vertex>> {
    let n = adj.len();
    let mut dist = vec![(std::i64::MAX, std::usize::MAX); n];
    dist[start].0 = 0;
    let mut updated = false;
    for _ in 0..n {
        for v in 0..n {
            if dist[v].0 == std::i64::MAX {
                continue;
            }
            for ref e in adj[v].iter() {
                if e.cap == 0 {
                    continue;
                }
                if dist[v].0 + e.cost < dist[e.to].0 {
                    dist[e.to] = (dist[v].0 + e.cost, v);
                    updated = true;
                    if v == start {
                        break;
                    }
                }
            }
        }
        if !updated {
            return None;
        }
    }

    let mut used = vec![false; n];
    let mut path = vec![start];
    used[start] = true;
    let mut crt = start;
    loop {
        path.push(dist[crt].1);
        crt = dist[crt].1;
        if used[crt] {
            break;
        }
        used[crt] = true;
    }
    let begin = (0..path.len()).filter(|&i| path[i] == crt).next().unwrap();
    let mut cycle = (&path[begin..]).to_vec();
    cycle.reverse();
    Some(cycle)
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
