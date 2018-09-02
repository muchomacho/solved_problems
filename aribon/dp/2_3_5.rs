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
    let a = read_vector::<usize>();
    let m = read_vector::<usize>();
    let k = read::<usize>();

    let mut dp = vec![vec![std::usize::MAX; k + 1]; n];
    dp[0][0] = 0;
    for i in 0..(k + 1) {
        if i >= a[0] && dp[0][i - a[0]] < m[0] {
            dp[0][i] = dp[0][i - a[0]] + 1;
        }
    }

    for i in 1..n {
        for j in 0..(k + 1) {
            if dp[i - 1][j] < std::usize::MAX {
                dp[i][j] = 0;
            } else if j >= a[i] && dp[i][j - a[i]] < m[i] {
                dp[i][j] = dp[i][j - a[i]] + 1;
            }
        }
    }

    if dp[n - 1][k] < std::usize::MAX {
        println!("Yes");
    } else {
        println!("No");
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