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

static G: f64 = 9.8;
static EPS: f64 = 0.000_000_000_1;

fn main() {
    let n = read::<usize>();
    let v = read::<f64>();
    let line = read_vector::<f64>();
    let (x, y) = (line[0], line[1]);
    let mut obs = vec![];
    for _ in 0..n {
        let line = read_vector::<f64>();
        if line[0] > x + EPS {
            continue;
        }
        obs.push((
            line[0],
            line[1],
            if line[2] > x { x } else { line[2] },
            line[3],
        ));
    }

    let n = obs.len();
    for i in 0..n {
        for coord in [
            (obs[i].0, obs[i].1),
            (obs[i].0, obs[i].3),
            (obs[i].2, obs[i].3),
            (obs[i].2, obs[i].1),
        ]
        .iter()
        {
            if check(
                Point {
                    x: coord.0,
                    y: coord.1,
                },
                Point { x, y },
                v,
                &obs,
            ) {
                println!("Yes");
                return;
            }
        }
    }
    if check(Point { x, y }, Point { x, y }, v, &obs) {
        println!("Yes");
        return;
    }
    println!("No");
}

fn check(p0: Point, p: Point, v: f64, obs: &[(f64, f64, f64, f64)]) -> bool {
    let a = 0.25 * G * G;
    let b = G * p0.y - v * v;
    let c = p0.x * p0.x + p0.y * p0.y;
    let mut d = b * b - 4.0 * a * c;
    if d < -EPS {
        return false;
    } else if d < 0.0 {
        d = 0.0;
    }
    'outer: for &sign in [1.0, -1.0].iter() {
        let t2 = (-b + sign * d.sqrt()) / (2.0 * a);
        let t = t2.sqrt();
        let v_x = p0.x / t;
        let v_y = (v * v - v_x * v_x).sqrt();
        let mid = (v_x * v_y) / G;
        let p_y = f(p.x, v_x, v_y);
        if p_y < p.y - EPS {
            continue;
        }
        'inner: for i in 0..obs.len() {
            if obs[i].0 <= p.x && obs[i].2 <= p.x && obs[i].3 <= p_y && obs[i].1 >= p.y {
                continue 'outer;
            }
            let (y_s, y_e) = (f(obs[i].0, v_x, v_y), f(obs[i].2, v_x, v_y));
            if !((y_s - obs[i].1) < -EPS && (y_e - obs[i].1) < -EPS)
                && !((y_s - obs[i].3) > EPS && (y_e - obs[i].3) > EPS)
            {
                continue 'outer;
            }
            if obs[i].0 <= mid && mid <= obs[i].2 {
                if y_s < obs[i].1 - EPS && f(mid, v_x, v_y) >= obs[i].1 + EPS {
                    continue 'outer;
                }
            }
        }
        return true;
    }
    false
}

fn f(x: f64, v_x: f64, v_y: f64) -> f64 {
    (v_y * x) / v_x - (G * x * x) / (2.0 * v_x * v_x)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
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
