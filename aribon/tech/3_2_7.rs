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
    let w = read_vector::<usize>();
    let v = read_vector::<usize>();
    let s = read::<usize>();

    if n == 1 {
        if w[0] <= s {
            println!("{}", v[0]);
        } else {
            println!("0");
        }
        return;
    }

    let half = n / 2;
    let mut scores = Vec::new();
    for i in 0..2usize.pow(half as u32) {
        let mut weight = 0;
        let mut val = 0;
        let mut bin = i;
        for j in 0..half {
            if bin & 1 == 1 {
                weight += w[j];
                val += v[j];
            }
            bin >>= 1;
        }
        if weight <= s {
            scores.push((weight, val));
        }
    }
    scores.sort_by(|a, b| a.0.cmp(&b.0));
    for i in 1..half {
        if scores[i].1 < scores[i - 1].1 {
            scores[i].1 = scores[i - 1].1;
        }
    }

    let mut answer = 0;
    for i in 0..2usize.pow((n - half) as u32) {
        let mut weight = 0;
        let mut val = 0;
        let mut bin = i;
        for j in half..n {
            if bin & 1 == 1 {
                weight += w[j];
                val += v[j];
            }
            bin >>= 1;
        }
        
        if weight > s {
            continue;
        }
        if let Some(v) = binary_search(s - weight, &scores) {
            if val + v > answer {
                answer = val + v;
            }
        }

    }

    println!("{}", answer);
}

fn binary_search(x: usize, vec: &[(usize, usize)]) -> Option<usize> {
    if x < vec[0].0 {
        return None;
    }
    if x >= vec[vec.len() - 1].0 {
        return Some(vec[vec.len() - 1].1);
    }

    let mut left = 0;
    let mut right = vec.len() - 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if x < vec[mid].0 {
            right = mid;
        } else {
            left = mid;
        }
    }

    Some(vec[left].1)

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
