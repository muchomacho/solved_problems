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
    let vec = read_vector::<i64>();
    let (a, b) = (vec[0], vec[1]);

    let (c, (x, y)) = ext_gcd(a, b);

    if c != 1 {
        println!("{}", -1);
    } else {
        if x < 0 {
            println!("0 {} {} 0", y, x * (-1));
        } else {
            println!("{} 0 0 {}", x, y * (-1));
        }
    }
}

fn ext_gcd(x: i64, y: i64) -> (i64, (i64, i64)) {
    let mut x = x;
    let mut y = y;
    let mut x_coef = (1, 0);
    let mut y_coef = (0, 1);

    loop {
        let residual = x % y;
        let quotient = x / y;
        if residual == 0 {
            break;
        }
        x = y;
        y = residual;
        let y_coef_new = (x_coef.0 - quotient * y_coef.0, x_coef.1 - quotient * y_coef.1);
        x_coef = y_coef;
        y_coef = y_coef_new;
    }

    return (y, y_coef);
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