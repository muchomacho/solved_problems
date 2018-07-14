
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};

fn main() {
    let n: usize = read();
    let r: usize = read();
    let mut x: Vec<usize> = read_vector();
    x.sort();

    let mut stone = 0;
    let mut present = 0;
    let mut count = 0;
    loop {
        for i in present..n {
            if x[present] + r < x[i] {
                stone = i - 1;
                count += 1;
                break;
            }
            if i == n - 1 {
                stone = n - 1;
                count += 1;
            }
        }
        for i in (stone + 1)..n {
            if x[stone] + r < x[i] {
                present = i;
                break;
            }
            if i == n - 1 {
                present = n;
            }
        }
        if stone == n - 1 || present == n {
            break;
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