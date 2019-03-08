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
    let m = read::<usize>();
    let n = read::<usize>();

    let mut vecs1 = vec![];
    for _ in 0..m {
        let line = read_vector::<i64>();
        vecs1.push(Vector(line[0], line[1]));
    }
    let ch1 = calc_convex_hull(&mut vecs1);
    let (u_path1, b_path1) = calc_path(&ch1);

    let mut vecs2 = vec![];
    for _ in 0..n {
        let line = read_vector::<i64>();
        vecs2.push(Vector(line[0], line[1]));
    }
    let ch2 = calc_convex_hull(&mut vecs2);
    let (u_path2, b_path2) = calc_path(&ch2);

    let mut x_start = max(ch1[0].0, ch2[0].0);
    let (mut u1, mut b1, mut u2, mut b2) = (
        (0..(u_path1.len() - 1))
            .filter(|&i| u_path1[i + 1].0 > x_start)
            .next()
            .unwrap(),
        (0..(b_path1.len() - 1))
            .filter(|&i| b_path1[i + 1].0 > x_start)
            .next()
            .unwrap(),
        (0..(u_path2.len() - 1))
            .filter(|&i| u_path2[i + 1].0 > x_start)
            .next()
            .unwrap(),
        (0..(b_path2.len() - 1))
            .filter(|&i| b_path2[i + 1].0 > x_start)
            .next()
            .unwrap(),
    );
    let mut ans = 0.0;
    let mut val_start = calc_area(
        x_start as f64,
        u1,
        b1,
        u2,
        b2,
        &u_path1,
        &b_path1,
        &u_path2,
        &b_path2
    );
    while u1 < u_path1.len() - 1 && u2 < u_path2.len() - 1 {
        let x_end = min(
            min(u_path1[u1 + 1].0, b_path1[b1 + 1].0),
            min(u_path2[u2 + 1].0, b_path2[b2 + 1].0),
        );
        let x_mid = (x_start as f64 + x_end as f64) / 2.0;
        let val_end = calc_area(
            x_end as f64,
            u1,
            b1,
            u2,
            b2,
            &u_path1,
            &b_path1,
            &u_path2,
            &b_path2
        );
        let val_mid = calc_area(
            x_mid,
            u1,
            b1,
            u2,
            b2,
            &u_path1,
            &b_path1,
            &u_path2,
            &b_path2
        );
        ans += ((x_end - x_start) as f64 / 6.0) * (val_start + 4.0 * val_mid + val_end);
        if u_path1[u1 + 1].0 == x_end {
            u1 += 1;
        }
        if b_path1[b1 + 1].0 == x_end {
            b1 += 1;
        }
        if u_path2[u2 + 1].0 == x_end {
            u2 += 1;
        }
        if b_path2[b2 + 1].0 == x_end {
            b2 += 1;
        }
        x_start = x_end;
        val_start = val_end;
    }

    println!("{}", ans);
}

fn calc_area(
    x: f64,
    u1: usize,
    b1: usize,
    u2: usize,
    b2: usize,
    u_path1: &[Vector],
    b_path1: &[Vector],
    u_path2: &[Vector],
    b_path2: &[Vector],
) -> f64 {
    calc_diff(x, u_path1[u1], u_path1[u1 + 1], b_path1[b1], b_path1[b1 + 1])
        * calc_diff(x, u_path2[u2], u_path2[u2 + 1], b_path2[b2], b_path2[b2 + 1])
}

fn calc_diff(x: f64, a1: Vector, a2: Vector, b1: Vector, b2: Vector) -> f64 {
    calc_val(x, a1, a2) - calc_val(x, b1, b2)
}

fn calc_val(x: f64, a: Vector, b: Vector) -> f64 {
    a.1 as f64 + ((a.1 as f64 - b.1 as f64) / (a.0 as f64 - b.0 as f64)) * (x - a.0 as f64)
}

fn calc_path(convex_hull: &[Vector]) -> (Vec<Vector>, Vec<Vector>) {
    let n = convex_hull.len();
    let (mut u_path, mut b_path) = (vec![convex_hull[0]], vec![convex_hull[0]]);
    let (mut crt_u, mut crt_b) = (0, 0);
    loop {
        let nxt_u = (crt_u + n - 1) % n;
        if convex_hull[nxt_u].0 < convex_hull[crt_u].0 {
            break;
        }
        u_path.push(convex_hull[nxt_u]);
        crt_u = nxt_u;
    }
    loop {
        let nxt_b = (crt_b + 1) % n;
        if convex_hull[nxt_b].0 < convex_hull[crt_b].0 {
            break;
        }
        b_path.push(convex_hull[nxt_b]);
        crt_b = nxt_b;
    }
    (u_path, b_path)
}

fn calc_convex_hull(vecs: &mut [Vector]) -> Vec<Vector> {
    vecs.sort();
    let n = vecs.len();
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
    convex_hull
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Vector(i64, i64);
impl Vector {
    #[allow(dead_code)]
    fn dot(self, rhs: Vector) -> i64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
    #[allow(dead_code)]
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
