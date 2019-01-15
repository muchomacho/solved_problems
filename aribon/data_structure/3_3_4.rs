#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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
    let m = read::<usize>();
    let a = read_vector::<usize>();
    let mut sorted = a.clone();
    let bucket_len = (n as f64 * (n as f64).log2()).sqrt() as usize;
    let bucket_num = n / bucket_len;
    let mut minimum = std::usize::MAX;
    let mut maximum = 0;
    for i in 0..(bucket_num + 1) {
        let l = i * bucket_len;
        let r = min((i + 1) * bucket_len, n);
        sorted[l..r].sort();
        if sorted[r - 1] > maximum {
            maximum = sorted[r - 1];
        }
        if sorted[l] < minimum {
            minimum = sorted[l];
        }
    }

    for _ in 0..m {
        let q = read_vector::<usize>();
        let (start, end, k) = (q[0] - 1, q[1] - 1, q[2]);
        if count(minimum, start, end, &a, &sorted, bucket_len) >= k {
            println!("{}", minimum);
            continue;
        }
        let (mut l, mut r) = (minimum, maximum);
        while r - l > 1 {
            let mid = (l + r) / 2;
            if count(mid, start, end, &a, &sorted, bucket_len) < k {
                l = mid;
            } else {
                r = mid;
            }
        }
        println!("{}", r);
    }
}

fn count(
    x: usize,
    start: usize,
    end: usize,
    array: &[usize],
    sorted: &[usize],
    bucket_len: usize,
) -> usize {
    let mut sum = 0;
    let start_index = (start as f64 / bucket_len as f64).ceil() as usize;
    let end_index = end / bucket_len;
    if start_index >= end_index {
        for i in start..(end + 1) {
            if array[i] <= x {
                sum += 1;
            }
        }
        return sum;
    }
    for i in start..(start_index * bucket_len) {
        if array[i] <= x {
            sum += 1;
        }
    }
    for i in (end_index * bucket_len)..(end + 1) {
        if array[i] <= x {
            sum += 1;
        }
    }
    for i in start_index..end_index {
        sum += match sorted[(i * bucket_len)..((i + 1) * bucket_len)].binary_search(&x) {
            Ok(num) => num + 1 - i * bucket_len,
            Err(num) => num - i * bucket_len,
        };
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
    return buf.trim().chars().collect();
}
