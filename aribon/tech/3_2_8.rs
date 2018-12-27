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

static DIRECTION: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let w = read::<usize>();
    let h = read::<usize>();
    let n = read::<usize>();
    let x1 = read_vector::<usize>().into_iter().map(|i| i - 1).collect::<Vec<_>>();
    let x2 = read_vector::<usize>().into_iter().map(|i| i - 1).collect::<Vec<_>>();
    let y1 = read_vector::<usize>().into_iter().map(|i| i - 1).collect::<Vec<_>>();
    let y2 = read_vector::<usize>().into_iter().map(|i| i - 1).collect::<Vec<_>>();

    let mut row = BTreeSet::new();
    row.insert(0);
    let mut column = BTreeSet::new();
    column.insert(0);
    for i in 0..n {
        row.insert(y1[i]);
        if y2[i] < h - 1 {
            row.insert(y2[i] + 1);
        }
        column.insert(x1[i]);
        if x2[i] < w - 1 {
            column.insert(x2[i] + 1);
        }
    }
    let mut row = row.into_iter().collect::<Vec<_>>();
    row.sort();
    let mut column = column.into_iter().collect::<Vec<_>>();
    column.sort();
    let mut graph = vec![vec![true; column.len()]; row.len()];
    for i in 0..n {
        if x1[i] == x2[i] {
            let c = column.binary_search(&x1[i]).unwrap();
            for j in 0..row.len() {
                if row[j] >= y1[i] && row[j] <= y2[i] {
                    graph[j][c] = false;
                }
            }
        } else {
            let r = row.binary_search(&y1[i]).unwrap();
            for j in 0..column.len() {
                if column[j] >= x1[i] && column[j] <= x2[i] {
                    graph[r][j] = false;
                }
            }
        }
    }

    for i in 0..row.len() {
        println!("{}", graph[i].iter().map(|i| if *i { "0" } else { "1" }).collect::<Vec<_>>().join(" "));
    }

    println!("{}", bfs(&mut graph));
}

fn bfs(graph: &mut [Vec<bool>]) -> usize {
    let (row, col) = (graph.len(), graph[0].len());
    let mut connected = 0;
    for i in 0..row {
        for j in 0..col {
            if !graph[i][j] {
                continue;
            }
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            graph[i][j] = false;
            while queue.len() > 0 {
                let (r, c) = queue.pop_front().unwrap();
                for &(dr, dc) in DIRECTION.iter() {
                    let (new_r, new_c) = (r as i64 + dr, c as i64 + dc);
                    if new_r >= 0 && new_r < row as i64 && new_c >= 0 && new_c < col as i64 {
                        let (new_r, new_c) = (new_r as usize, new_c as usize);
                        if graph[new_r][new_c] {
                            queue.push_back((new_r, new_c));
                            graph[new_r][new_c] = false;
                        }
                    }
                }
            }
            connected += 1;
        }
    }

    return connected;
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
