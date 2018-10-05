#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap, HashMap};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write, BufReader, BufWriter};
#[allow(unused_imports)]
use std::io::prelude::BufRead;


fn main() {
    let n = read::<usize>();
    if n <= 2 {
        println!("Yes");
        return;
    }
    let mut adjacency_list = vec![vec![]; n];
    let k = read::<usize>();
    for _ in 0..k {
        let param = read_vector::<usize>();
        let (a, b) = (param[0], param[1]);
        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    let mut colors = vec![0; n];
    for i in 0..n {
        if colors[i] == 0 {
            colors[i] = 1;
            bfs(i, n, &adjacency_list, &mut colors);
        }
    }

    for i in 0..n {
        for &j in adjacency_list[i].iter() {
            if colors[i] == colors[j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");

}

fn bfs(s: usize, n: usize, list: &Vec<Vec<usize>>, colors: &mut Vec<i64>) {
    let mut used = vec![false; n];
    let mut queue = VecDeque::new();
    used[s] = true;
    queue.push_back(s);
    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();
        for neighbor in list[node].iter() {
            if used[*neighbor] {
                continue;
            }
            used[*neighbor] = true;
            colors[*neighbor] = colors[node] * (-1);
            queue.push_back(*neighbor);
        }
    }
}

#[allow(dead_code)]
fn read<T>() -> T where T: std::str::FromStr, T::Err: std::fmt::Debug {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return buf
        .trim()
        .parse()
        .unwrap();
}

#[allow(dead_code)]
fn read_vector<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    let mut buf = String::with_capacity(100);
    stdin().read_line(&mut buf).unwrap();
    return buf
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}

#[allow(dead_code)]
fn read_matrix<T>() -> Vec<Vec<T>> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    use std::io::prelude::*;
    let stdin = stdin();
    let mut reader = BufReader::with_capacity(100 * 1024, stdin);
    let mut line = String::with_capacity(100);
    let mut matrix: Vec<Vec<T>> = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        matrix.push(line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect());
        line.clear();
    }

    return matrix;
}