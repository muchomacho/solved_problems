#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::prelude::BufRead;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::str::FromStr;

static MAX: usize = 1_000_000_000;

fn main() {
    let _n = read::<usize>();
    let m = read::<usize>();
    let mut x = read_vector::<usize>();
    x.sort();

    if count(MAX, &x) >= m {
        println!("{}", MAX);
        return;
    } else if count(1, &x) < m {
        println!("0");
        return;
    }

    let (mut left, mut right) = (1, MAX);
    while right - left > 1 {
        let mid = (left + right) / 2;
        if count(mid, &x) >= m {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);

}

fn count(minimum: usize, x: &[usize]) -> usize {
    let mut sum = 1;
    let mut last_x = x[0];
    for i in 1..x.len() {
        if x[i] - last_x >= minimum {
            sum += 1;
            last_x = x[i];
        }
    }

    sum
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
    return buf.chars().collect();
}
