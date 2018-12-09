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
    let n = read::<usize>();

    if is_prime(n) {
        println!("No");
        return;
    }

    for i in 2..n {
        if mod_power(i, n) == i {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn is_prime(n: usize) -> bool {
    if n <= 2 {
        return true;
    }
    let root_n = (n as f64).sqrt() as usize;
    for i in 2..(root_n + 1) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn convert_to_binary(n: usize) -> Vec<usize> {
    let mut n_mut = n;
    let n_log = (n as f64).log2() as usize;
    let mut binary_exp = vec![0; n_log + 1];
    for i in 0..n_log {
        binary_exp[i] = n_mut % 2;
        n_mut = (n_mut - n_mut % 2) / 2;
    }
    binary_exp[n_log] = 1;

    return binary_exp;
}

fn power(x: usize, n: usize) -> usize {
    let binary_exp = convert_to_binary(n);
    let mut binary_pow = vec![0; binary_exp.len()];
    binary_pow[0] = x;
    for i in 1..binary_exp.len() {
        binary_pow[i] = binary_pow[i - 1] * binary_pow[i - 1];
    }

    return (0..binary_exp.len()).fold(1, |acc, x| {
        if binary_exp[x] == 1 {
            acc * binary_pow[x]
        } else {
            acc
        }
    });
}

fn mod_power(x: usize, n: usize) -> usize {
    let binary_exp = convert_to_binary(n);
    let mut binary_pow = vec![0; binary_exp.len()];
    binary_pow[0] = x;
    for i in 1..binary_exp.len() {
        binary_pow[i] = (binary_pow[i - 1] * binary_pow[i - 1]) % n;
    }

    return (0..binary_exp.len()).fold(1, |acc, x| {
        if binary_exp[x] == 1 {
            (acc * binary_pow[x]) % n
        } else {
            acc
        }
    });
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
