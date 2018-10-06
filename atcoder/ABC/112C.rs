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
    let mut heights = vec![vec![vec![std::i64::MAX]; 101]; 101];
    let n = read::<usize>();
    for _ in 0..n {
        let vec = read_vector::<i64>();
        let (x, y, h) = (vec[0], vec[1], vec[2]);
        for i in 0..101 {
            for j in 0..101 {
                if heights[i][j][0] == -1 {
                    continue;
                }
                let height = h + (i as i64 - x).abs() + (j as i64 - y).abs();
                if h == 0 {
                    heights[i][j].push(height);
                } else if heights[i][j][0] == std::i64::MAX {
                    heights[i][j][0] = height;
                } else if heights[i][j][0] != height {
                    heights[i][j][0] = -1;
                }
            }
        }
    }
 
    let mut cand = vec![];
    for i in 0..101 {
        'outer: for j in 0..101 {
            if heights[i][j][0] == -1 {
                continue;
            }
            for k in 0..heights[i][j].len() {
                if heights[i][j][0] > heights[i][j][k] {
                    continue 'outer;
                }
            }
 
            cand.push((i, j));
        }
    }
 
    if cand.len() == 1 {
        let (i, j) = (cand[0].0, cand[0].1);
        println!("{} {} {}", i, j, heights[i][j][0]);
        return;
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