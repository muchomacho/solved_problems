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
    let mut lines = vec![];
    for _ in 0..n {
        let line = read_vector::<f64>();
        let (p1, p2) = (
            Point {
                x: line[0],
                y: line[1],
            },
            Point {
                x: line[2],
                y: line[3],
            },
        );
        let l = Line::new(p1, p2).unwrap();
        lines.push(((p1, p2), l));
    }
    let mut union_find = UnionFindTree::new(n);
    for i in 0..n {
        for j in (i + 1)..n {
            match lines[i].1.cross_point(&lines[j].1) {
                CrossType::CrossPoint { coord } => {
                    if have_overlap((coord, coord), lines[i].0)
                        && have_overlap((coord, coord), lines[j].0)
                    {
                        union_find.union(i, j);
                    }
                }
                CrossType::Same => {
                    if have_overlap(lines[i].0, lines[j].0) {
                        union_find.union(i, j);
                    }
                }
                CrossType::None => {}
            }
        }
    }

    let m = read::<usize>();
    for _ in 0..m {
        let line = read_vector::<usize>();
        if union_find.in_same_group(line[0] - 1, line[1] - 1) {
            println!("CONNECTED");
        } else {
            println!("NOT CONNECTED");
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// ax + by + c = 0
#[derive(Clone, Copy, Debug, PartialEq)]
struct Line {
    x_coef: f64,
    y_coef: f64,
    bias: f64,
}

#[derive(Clone, Copy, Debug)]
enum CrossType {
    CrossPoint { coord: Point },
    Same,
    None,
}

impl Line {
    fn new(a: Point, b: Point) -> Option<Line> {
        if a == b {
            return None;
        }
        let x_coef;
        let y_coef;
        let bias;
        if a.x == b.x {
            x_coef = 1.0;
            y_coef = 0.0;
            bias = -a.x;
        } else if a.y == b.y {
            x_coef = 0.0;
            y_coef = 1.0;
            bias = -a.y;
        } else {
            x_coef = a.y - b.y;
            y_coef = -(a.x - b.x);
            bias = x_coef * a.x * (-1.0) + y_coef * a.y * (-1.0);
        }
        Some(Line {
            x_coef,
            y_coef,
            bias,
        })
    }

    fn cross_point(&self, l: &Line) -> CrossType {
        if *self == *l {
            CrossType::Same
        } else if self.x_coef == 0.0 {
            if l.x_coef == 0.0 {
                CrossType::None
            } else {
                let y = -self.bias;
                let x = -(l.y_coef * y + l.bias) / l.x_coef;
                CrossType::CrossPoint {
                    coord: Point { x, y },
                }
            }
        } else if self.y_coef == 0.0 {
            if l.y_coef == 0.0 {
                CrossType::None
            } else {
                let x = -self.bias;
                let y = -(l.x_coef * x + l.bias) / l.y_coef;
                CrossType::CrossPoint {
                    coord: Point { x, y },
                }
            }
        } else {
            if l.x_coef == 0.0 || l.y_coef == 0.0 {
                l.cross_point(self)
            } else {
                let y_coef_diff = (self.y_coef * l.x_coef - self.x_coef * l.y_coef) / l.x_coef;
                let bias_diff = -(self.bias * l.x_coef - self.x_coef * l.bias) / l.x_coef;
                if y_coef_diff == 0.0 {
                    if bias_diff == 0.0 {
                        CrossType::Same
                    } else {
                        CrossType::None
                    }
                } else {
                    let y = bias_diff / y_coef_diff;
                    let x = -(self.y_coef * y + self.bias) / self.x_coef;
                    CrossType::CrossPoint {
                        coord: Point { x, y },
                    }
                }
            }
        }
    }
}

struct UnionFindTree {
    parents: Vec<usize>,
    heights: Vec<usize>,
}

impl UnionFindTree {
    fn new(size: usize) -> UnionFindTree {
        UnionFindTree {
            parents: (0..size).collect(),
            heights: vec![0; size],
        }
    }

    fn climb_up(&mut self, node: usize) -> usize {
        let mut parents_to_root = vec![node];
        let mut present = node;

        loop {
            if self.parents[present] == present {
                for node in parents_to_root.iter() {
                    self.parents[*node] = present;
                }
                return present;
            }
            present = self.parents[present];
            parents_to_root.push(present);
        }
    }

    fn in_same_group(&mut self, x: usize, y: usize) -> bool {
        self.climb_up(x) == self.climb_up(y)
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.climb_up(x);
        let root_y = self.climb_up(y);

        if self.heights[root_x] > self.heights[root_y] {
            self.parents[root_y] = root_x;
        } else if self.heights[root_x] < self.heights[root_y] {
            self.parents[root_x] = root_y
        } else {
            self.parents[root_y] = root_x;
            self.heights[root_x] += 1;
        }
    }
}

fn have_overlap(a: (Point, Point), b: (Point, Point)) -> bool {
    let a_x_min = if a.0.x <= a.1.x { a.0.x } else { a.1.x };
    let a_x_max = if a.0.x >= a.1.x { a.0.x } else { a.1.x };
    let a_y_min = if a.0.y <= a.1.y { a.0.y } else { a.1.y };
    let a_y_max = if a.0.y >= a.1.y { a.0.y } else { a.1.y };
    let b_x_min = if b.0.x <= b.1.x { b.0.x } else { b.1.x };
    let b_x_max = if b.0.x >= b.1.x { b.0.x } else { b.1.x };
    let b_y_min = if b.0.y <= b.1.y { b.0.y } else { b.1.y };
    let b_y_max = if b.0.y >= b.1.y { b.0.y } else { b.1.y };

    let x_overlap = a_x_min <= b_x_max && a_x_max >= b_x_min;
    let y_overlap = a_y_min <= b_y_max && a_y_max >= b_y_min;

    x_overlap && y_overlap
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
