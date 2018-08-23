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
    let w = read_vector::<usize>();
    let v = read_vector::<usize>();
    let m = read::<usize>();

    let v_max = v.iter().fold(0, |acc, &i| acc + i);

    let mut dp: Vec<Vec<usize>> = vec![vec![std::usize::MAX; v_max + 1]; n];
    dp[0][v[0]] = w[0];
    for i in 1..n {
        for j in 0..(v_max + 1) {
            dp[i][j] = dp[i - 1][j];
            if j >= v[i] && dp[i - 1][j - v[i]] < std::usize::MAX {
                dp[i][j] = min(dp[i][j], dp[i - 1][j - v[i]] + w[i]);
            }
        }
    }

    for j in (0..(v_max + 1)).rev() {
        if dp[n - 1][j] <= m {
            println!("{}", dp[n - 1][j]);
            break;
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