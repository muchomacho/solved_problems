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

static EPS: f64 = 0.000_000_000_1;

fn main() {
    let n = read::<usize>();
    let mut circles = vec![];
    let mut boundaries = vec![];
    for i in 0..n {
        let line = read_vector::<f64>();
        circles.push((Point { x: line[0], y: line[1] }, line[2]));
        boundaries.push(SetItem(line[0] - line[2], i));
        boundaries.push(SetItem(line[0] + line[2], n + i));
    }
    boundaries.sort();

    let mut outers = vec![];
    let mut contacts: BTreeSet<SetItem> = BTreeSet::new();
    for i in 0..boundaries.len() {
        let index = boundaries[i].1 % n;
        if boundaries[i].1 < n {
            let mut contained = false;
            if let Some(item) = contacts.range(..SetItem(circles[index].0.y, std::usize::MAX)).next_back() {
                if contains(circles[item.1 % n].0, circles[item.1 % n].1, circles[index].0) {
                    contained = true;
                }
            }
            if let Some(item) = contacts.range(SetItem(circles[index].0.y, 0)..).next() {
                if contains(circles[item.1 % n].0, circles[item.1 % n].1, circles[index].0) {
                    contained = true;
                }
            }
            if !contained {
                contacts.insert(boundaries[i]);
                outers.push(index);
            }
        } else {
            contacts.remove(&boundaries[i]);
        }
    }

    println!("{}", outers.len());
    for i in 0..outers.len() {
        if i < outers.len() - 1 {
            print!("{} ", outers[i]);
        } else {
            println!("{}", outers[i]);
        }
    }
}

fn contains(p1: Point, r1: f64, p2: Point) -> bool {
    let dist = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt();
    dist < r1 - EPS
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct SetItem(f64, usize);
impl Eq for SetItem {}
impl Ord for SetItem {
    fn cmp(&self, other: &SetItem) -> Ordering {
        if self.0 < other.0 {
            return Ordering::Less;
        }
        if self.0 > other.0 {
            return Ordering::Greater;
        }
        self.1.cmp(&other.1)
    }
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
