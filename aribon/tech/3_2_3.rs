#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::prelude::BufRead;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    let n = read::<usize>();
    let s = read_chars().into_iter().map(|ch| if ch == 'F' { true } else { false }).collect::<Vec<_>>();;

    let mut minimum_k = 1;
    let mut minimum = operation_count(1, &s).unwrap();
    if n == 1 {
        println!("{}", minimum_k);
        println!("{}", minimum);
        return;
    }
    for i in 2..(n + 1) {
        if let Some(x) = operation_count(i, &s) {
            if x < minimum {
                minimum_k = i;
                minimum = x;
            }
        }
    }

    println!("{}", minimum_k);
    println!("{}", minimum);
}

fn present_state(i: usize, total: &mut usize, operation: &[usize], k: usize, s: &[bool]) -> bool {
    if i > 0 {
        *total += operation[i - 1];
    }
    if i >= k {
        *total -= operation[i - k];
    }

    if *total % 2 == 0 {
        s[i]
    } else {
        !s[i]
    }
}

fn operation_count(k: usize, s: &[bool]) -> Option<usize> {
    let mut operation = vec![0; s.len()];
    let mut total_operation = 0;
    let mut count = 0;
    for i in 0..(s.len() - k + 1) {
        if !present_state(i, &mut total_operation, &operation, k, s) {
            count += 1;
            operation[i] = 1;
        }
    }

    for i in (s.len() - k + 1)..s.len() {
        if !present_state(i, &mut total_operation, &operation, k, s) {
            return None;
        }
    }

    Some(count)
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
    return buf.trim().chars().collect();
}
