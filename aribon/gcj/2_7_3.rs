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
    let p = read::<usize>();
    let q = read::<usize>();
    let mut a = read_vector::<usize>();
    a.insert(0, 0);
    a.push(p + 1);

    let mut dp = vec![vec![std::usize::MAX; q + 2]; q + 2];

    println!("{}", recursion(&a, &mut dp, 0, q + 1));
}

fn recursion(release: &Vec<usize>, dp: &mut Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    if dp[x][y] < std::usize::MAX {
        return dp[x][y];
    }
    
    if x + 1 == y {
        dp[x][y] = 0;
        return 0;
    }

    let cost = release[y] - release[x] - 2;
    dp[x][y] = ((x + 1)..y).fold(std::usize::MAX, |acc, i| {
        min(
            acc,
            cost + recursion(release, dp, x, i) + recursion(release, dp, i, y),
        )
    });
    return dp[x][y];
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
