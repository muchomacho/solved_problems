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

fn main() {
    let a = read::<usize>();
    let b = read::<usize>();

    println!("{}", ext_eratosthenes(a, b));
}

fn ext_eratosthenes(a: usize, b: usize) -> usize {
    let root_b = (b as f64).sqrt() as usize;
    let mut array = vec![true; root_b + 1];
    array[0] = false;
    array[1] = false;
    let mut interval = vec![true; b - a + 1];
    if a == 1 {
        interval[0] = false;
    }
    for i in 2..(root_b + 1) {
        if array[i] {
            let quotient = root_b / i;
            if quotient > 1 {
                for j in 2..(quotient + 1) {
                    array[i * j] = false;
                }
            }
            let start = if a % i == 0 { 0 } else { i - a % i };
            let quotient = (b - a - start) / i;
            for j in 0..(quotient + 1) {
                interval[start + i * j] = false;
            }
            if a + start == i {
                interval[start] = true;
            }
        }
    }

    return interval.iter().fold(0, |acc, x| if *x { acc + 1 } else { acc });
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
