
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};

fn main() {
    let _n: usize = read();
    let mut list: BinaryHeap<i32> = read_vector::<i32>()
        .into_iter()
        .map(|i| - i)
        .collect();
    let mut cost = 0;
    while list.len() > 1 {
        let min1 = list.pop().unwrap() * (-1);
        let min2 = list.pop().unwrap() * (-1);
        cost += min1 + min2;
        list.push(-(min1 + min2));
    }
    println!("{}", cost);
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