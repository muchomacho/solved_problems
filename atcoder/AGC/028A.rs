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
    let vec = read_vector::<usize>();
    let mut n = vec[0];
    let mut m = vec[1];
    let mut s = read_chars();
    let mut t = read_chars();

    if n > m {
        swap(&mut n, &mut m);
        swap(&mut s, &mut t);
    }

    let div = gcd(n, m);

    for i in 0..n {
        if (m * i) % n != 0 {
            continue;
        }
        let j = (m * i) / n;
        if s[i] != t[j] {
            println!("-1");
            return;
        }
    }

    println!("{}", div * (n / div) * (m / div));
    
}

fn gcd(a: usize, b: usize) -> usize {
    let mut x = b;
    let mut y = a;
    loop {
        let tmp = x % y;
        if tmp == 0 {
            return y;
        }
        x = y;
        y = tmp;
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

#[allow(dead_code)]
fn read_chars() -> Vec<char> {
    let stdin = stdin();
    let mut buf = String::new();
    let _bytes = stdin.read_line(&mut buf).unwrap();
    return buf.chars().collect();
}