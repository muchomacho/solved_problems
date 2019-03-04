#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
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
    let mut vecs = vec![];
    for _ in 0..n {
        let line = read_vector::<i64>();
        vecs.push(Vector(line[0], line[1]));
    }
    vecs.sort();

    let mut convex_hull = vec![vecs[0]];
    for i in 1..n {
        while convex_hull.len() >= 2 {
            let len = convex_hull.len();
            let last_2 = convex_hull[len - 2];
            let last = convex_hull[len - 1];
            if (last - last_2).cross_prod(vecs[i] - last) <= 0 {
                convex_hull.pop().unwrap();
            } else {
                break;
            }
        }
        convex_hull.push(vecs[i]);
    }
    let size = convex_hull.len();
    for i in (0..(n - 1)).rev() {
        while convex_hull.len() > size {
            let len = convex_hull.len();
            let last_2 = convex_hull[len - 2];
            let last = convex_hull[len - 1];
            if (last - last_2).cross_prod(vecs[i] - last) <= 0 {
                convex_hull.pop().unwrap();
            } else {
                break;
            }
        }
        if i == 0 {
            break;
        }
        convex_hull.push(vecs[i]);
    }
    let (l, r) = (
        (1..convex_hull.len()).fold(0, |acc, i| {
            if convex_hull[i] < convex_hull[acc] {
                i
            } else {
                acc
            }
        }),
        (1..convex_hull.len()).fold(0, |acc, i| {
            if convex_hull[i] > convex_hull[acc] {
                i
            } else {
                acc
            }
        }),
    );
    let size = convex_hull.len();
    let mut max_dist = 0;
    let (mut l_crt, mut r_crt) = (l, r);
    loop {
        let (l_nxt, r_nxt) = ((l_crt + 1) % size, (r_crt + 1) % size);
        if (convex_hull[l_nxt] - convex_hull[l_crt]).cross_prod(convex_hull[r_crt] - convex_hull[r_nxt]) < 0 {
            r_crt = r_nxt;
        } else {
            l_crt = l_nxt;
        }
        if l_crt == r && r_crt == l {
            break;
        }
        max_dist = max(max_dist, (convex_hull[l_crt] - convex_hull[r_crt]).dot(convex_hull[l_crt] - convex_hull[r_crt]));
    }

    println!("{}", max_dist);
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Vector(i64, i64);
impl Vector {
    fn dot(self, rhs: Vector) -> i64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    fn cross_prod(self, rhs: Vector) -> i64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }
}
impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
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
