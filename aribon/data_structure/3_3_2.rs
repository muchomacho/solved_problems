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
    let a = read_vector::<i32>();

    let mut ans = 0;
    let init = vec![0; n + 1];
    let mut bit = BinaryIndexedTree::new(&init);
    for i in 0..n {
        ans += bit.query(n) - bit.query(a[i] as usize);
        bit.update(a[i] as usize, 1);
    }

    println!("{}", ans);
}

struct BinaryIndexedTree {
    array: Vec<i32>,
    len: usize,
    size: usize,
}

impl BinaryIndexedTree {
    fn new(values: &[i32]) -> BinaryIndexedTree {
        let len = values.len();
        let size = 2usize.pow((len as f64).log2().ceil() as u32);
        let array = vec![0; size];
        let mut bit = BinaryIndexedTree{ array, len, size };
        for i in 0..len {
            bit.update(i, values[i]);
        }
        bit
    }

    fn update(&mut self, i: usize, dx: i32) {
        assert!(i < self.len);
        let mut i = i as i32 + 1;
        while (i as usize) <= self.size {
            self.array[i as usize - 1] += dx;
            i += i & (-i);
        }
    }

    fn query(&self, i: usize) -> i32 {
        assert!(i < self.len);
        let mut i = i as i32 + 1;
        let mut sum = 0;
        while i > 0 {
            sum += self.array[i as usize - 1];
            i -= i & (-i);
        }
        sum
    }
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
