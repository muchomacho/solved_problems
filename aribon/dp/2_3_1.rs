
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap, HashMap};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use std::io::prelude::BufRead;


fn main() {
    let n = read::<usize>();
    let w = read_vector::<usize>();
    let v = read_vector::<usize>();
    let m = read::<usize>();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; n]; m + 1];
    for i in 1..(m + 1) {
        if i >= w[0] {
            dp[i][0] = v[0];
        }
    }
    for i in 1..n {
        for j in 1..(m + 1) {
            if j >= w[i] {
                dp[j][i] = max(max(dp[j][i - 1], dp[j - 1][i]), dp[j - w[i]][i - 1] + v[i]);
            } else {
                dp[j][i] = max(dp[j][i - 1], dp[j - 1][i]);
            }
        }
    }

    println!("{}", dp[m][n - 1]);
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
    let mut buf = String::new();
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
    return stdin
        .lock()
        .lines()
        .map(|line| line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect())
        .collect();
}