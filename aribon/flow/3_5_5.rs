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

static DIRECTION: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let x = read::<usize>();
    let y = read::<usize>();
    let mut graph = vec![vec![0; y]; x];
    let mut starts = vec![];
    let mut goals = vec![];
    for i in 0..x {
        let line = read_chars();
        for j in 0..y {
            if line[j] == '.' {
                graph[i][j] = 1;
                starts.push((i, j));
            } else if line[j] == 'D' {
                graph[i][j] = 2;
                goals.push((i, j));
            }
        }
    }

    let mut time_to_pair: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for &start in starts.iter() {
        let start_index = starts.binary_search(&start).unwrap();
        let times = bfs(start, &graph);
        if times.len() == 0 {
            println!("impossible");
            return;
        }
        for &(goal, t) in times.iter() {
            let goal_index = goals.binary_search(&goal).unwrap();
            if time_to_pair.contains_key(&t) {
                time_to_pair.get_mut(&t).unwrap().push((start_index, goal_index));
            } else {
                time_to_pair.insert(t, vec![(start_index, goal_index)]);
            }
        }
    }

    let s_offset = 2;
    let mut g_offset = starts.len() + 2;
    let mut adj = vec![HashMap::new(); starts.len() + 2];
    let (src, snk) = (0, 1);
    for i in 0..starts.len() {
        adj[src].insert(i + s_offset, 1);
        adj[i + s_offset].insert(src, 0);
    }
    let mut time = 1;
    let mut goaled = 0;
    let mut reachable = vec![vec![]; starts.len()];
    while goaled < starts.len() {
        time += 1;
        for g in 0..goals.len() {
            adj.push(HashMap::new());
            adj[g + g_offset].insert(snk, 1);
            adj[snk].insert(g + g_offset, 0);
        }
        if time_to_pair.contains_key(&time) {
            for &(s, g) in time_to_pair.get(&time).unwrap().iter() {
                reachable[s].push(g);
            }
        }
        for s in 0..starts.len() {
            for &g in reachable[s].iter() {
                adj[s + s_offset].insert(g + g_offset, 1);
                adj[g + g_offset].insert(s + s_offset, 0);
            }
        }
        let new_goaled = bipartite_matching(src, snk, &mut adj);
        goaled += new_goaled;
        g_offset += goals.len();
    }

    println!("{}", time);
}

fn bfs(src: (usize, usize), graph: &Vec<Vec<usize>>) -> Vec<((usize, usize), usize)> {
    let (r, c) = (graph.len(), graph[0].len());
    let mut used = vec![vec![false; c]; r];
    let mut stack = VecDeque::new();
    stack.push_back((src, 0));
    let mut dist = vec![];
    while stack.len() > 0 {
        let p = stack.pop_front().unwrap();
        let (point, time) = (p.0, p.1);
        if graph[point.0][point.1] == 2 {
            dist.push(((point.0, point.1), time));
        }
        for d in DIRECTION.iter() {
            if point.0 as i32 + d.0 < 0 || point.0 as i32 + d.0 >= r as i32 || point.1 as i32 + d.1 < 0 || point.1 as i32 + d.1 >= c as i32 {
                continue;
            }
            let (new_x, new_y) = ((point.0 as i32 + d.0) as usize, (point.1 as i32 + d.1) as usize);
            if used[new_x][new_y] || graph[new_x][new_y] == 0 {
                continue;
            }
            stack.push_back(((new_x, new_y), time + 1));
            used[new_x][new_y] = true;
        }
    }

    dist
}

fn bipartite_matching(src: usize, snk: usize, adj: &mut [HashMap<usize, usize>]) -> usize {
    let mut matched = 0;
    while let Some(path) = dfs(src, snk, &adj) {
        for i in 0..(path.len() - 1) {
            *adj[path[i]].get_mut(&path[i + 1]).unwrap() -= 1;
            *adj[path[i + 1]].get_mut(&path[i]).unwrap() += 1;
        }
        matched += 1;
    }
    matched
}

fn dfs(src: usize, snk: usize, adj: &[HashMap<usize, usize>]) -> Option<Vec<usize>> {
    let mut used = vec![false; adj.len()];
    let mut stack = VecDeque::new();
    let mut used_stack = VecDeque::new();
    stack.push_front((src, src));
    while stack.len() > 0 {
        let p = stack.pop_front().unwrap();
        let crt = p.0;
        used_stack.push_front(p);
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
