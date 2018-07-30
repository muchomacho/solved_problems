
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

    let mut dp: Vec<Vec<usize>> = vec![vec![0; m + 1]; n];
    for i in 1..(m + 1) {
        dp[0][i] = (i / w[0]) * v[0];
    }
    for item in 1..n {
        for weight in 1..(m + 1) {
            dp[item][weight] = dp[item - 1][weight];
            if weight >= w[item] {
                dp[item][weight] = max(dp[item][weight], dp[item][weight - w[item]] + v[item]);
            }
        }
    }
    println!("{}", dp[n - 1][m]);
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