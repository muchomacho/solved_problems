
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap, HashMap};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};


fn main() {
    let t: usize = read();
    let query = read_matrix::<i64>();

    for i in 0..t {
        let (a, b, c, d) = (query[i][0], query[i][1], query[i][2], query[i][3]);

        if a < b {
            println!("No");
            continue;
        } else if d < b {
            println!("No");
            continue;
        } else if c >= b {
            println!("Yes");
            continue;
        } else {
            let g = gcd(&b, &d);
            let maximum = b - g + a % g;
            if maximum > c {
                println!("No");
                continue;
            } else {
                println!("Yes");
            }
        }
    }
}

fn gcd(a: &i64, b: &i64) -> i64 {
    let (mut a, mut b) = (min(*a, *b), max(*a, *b));
    let answer;
    loop {
        b = b % a;
        if b == 0 {
            answer = a;
            break;
        }
        swap(&mut a, &mut b);
    }
    return answer;
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