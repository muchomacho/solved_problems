#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};


fn main() {
    let n: usize = read();
    let colors = read_vector::<usize>();

    let mut former = 0;
    let mut count = 0;
    for i in 0..n {
        if colors[i] == former {
            count += 1;
            former = 0;
        } else {
            former = colors[i];
        }
    }

    println!("{}", count);
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