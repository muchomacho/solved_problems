#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::prelude::BufRead;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    let m = read::<usize>();
    let n = read::<usize>();
    let mut graph = Vec::new();
    for _ in 0..m {
        graph.push(read_vector::<usize>().iter().map(|i| *i == 1).collect::<Vec<_>>());
    }
    
    let mut minimum = std::usize::MAX;
    let mut minimum_operation = vec![];
    for i in 0..(2usize.pow(n as u32)) {
        let mut colors = graph.clone();
        let mut operation = Vec::new();
        let mut bin = format!("{:b}", i).chars().collect::<Vec<_>>();
        bin.reverse();
        for j in (n - bin.len())..n {
            if bin[j - n + bin.len()] == '1' {
                operation.push((0, j));
                paint(0, j, &mut colors);
            }
        }
        for i in 1..m {
            for j in 0..n {
                if colors[i - 1][j] {
                    operation.push((i, j));
                    paint(i, j, &mut colors);
                }
            }
        }
        let mut correct = true;
        for j in 0..n {
            if colors[m - 1][j] {
                correct = false;
                break;
            }    
        }
        if !correct {
            continue;
        }
        if operation.len() < minimum {
            minimum = operation.len();
            minimum_operation = operation.clone();
        }
    }

    let mut answer = vec![vec![0; n]; m];
    for &(i, j) in minimum_operation.iter() {
        answer[i][j] = 1;
    }

    for row in answer {
        let row_string = row.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
        println!("{}", row_string);
    }

}

fn paint(i: usize, j: usize, graph: &mut Vec<Vec<bool>>) {
    graph[i][j] = !graph[i][j];
    if i > 0 {
        graph[i - 1][j] = !graph[i - 1][j];
    }
    if i < graph.len() - 1 {
        graph[i + 1][j] = !graph[i + 1][j];
    }
    if j > 0 {
        graph[i][j - 1] = !graph[i][j - 1];
    }
    if j < graph[0].len() - 1 {
        graph[i][j + 1] = !graph[i][j + 1];
    }
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
