
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{VecDeque, HashSet, BTreeSet, BinaryHeap, HashMap};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use std::io::prelude::BufRead;


static DIRECTION: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let param = read_vector::<usize>();
    let (h, w, t) = (param[0], param[1], param[2]);
    let stdin = stdin();
    let grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|line| line
            .unwrap()
            .chars()
            .collect())
        .collect();

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'S' {
                start = (i, j);
            } else if grid[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }

    let mut minimum: usize = 1;
    let mut maximum: usize = 1_000_000_000;
    let mut answer = 0;
    loop {
        if maximum == minimum + 1 {
            let time = dijkstra(&grid, &start, &goal, maximum);
            if time <= t {
                answer = maximum;
            } else {
                answer = minimum;
            }
            break;
        }

        let threshold = (minimum + maximum) / 2;
        let time = dijkstra(&grid, &start, &goal, threshold);
        if maximum - minimum == 2 {

        }
        if time < t {
            minimum = threshold;
        } else if time > t {
            maximum = threshold;
        } else {
            answer = threshold;
            break;
        }

    }

    println!("{}", answer);
}

fn dijkstra(matrix: &Vec<Vec<char>>, start: &(usize, usize), goal: &(usize, usize), c: usize) -> usize {
    let (h, w) = (matrix.len(), matrix[0].len());
    let mut used = vec![vec![false; w]; h];
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    heap.push(get_val(start, 0));

    while heap.len() > 0 {
        let (x, y, cost) = decom_val(heap.pop().unwrap());
        if used[x][y] {
            continue;
        }
        if x == goal.0 && y == goal.1 {
            return cost;
        }
        used[x][y] = true;

        for dir in DIRECTION.iter() {
            if let Some((new_x, new_y)) = get_coord(&(x, y), dir, h, w) {
                if used[new_x][new_y] {
                    continue;
                }
                if matrix[new_x][new_y] == '.' || matrix[new_x][new_y] == 'G' {
                    heap.push(get_val(&(new_x, new_y), cost + 1));
                } else if matrix[new_x][new_y] == '#' {
                    heap.push(get_val(&(new_x, new_y), cost + c));
                }
            }
        }
    }
    return 0;
}

fn get_val(point: &(usize, usize), cost: usize) -> i64 {
    return (cost * 100 + point.0 * 10 + point.1) as i64 * (-1);
}

fn decom_val(val: i64) -> (usize, usize, usize) {
    let val = (val * (-1)) as usize;
    return ((val / 10) % 10, val % 10, val / 100);
}

fn get_coord(point: &(usize, usize), dir: &(i64, i64), h: usize, w: usize) -> Option<(usize, usize)> {
    let (new_x, new_y) = (point.0 as i64 + dir.0, point.1 as i64 + dir.1);
    if new_x >= 0 && new_x < h as i64 && new_y >= 0 && new_y < w as i64 {
        return Some((new_x as usize , new_y as usize));
    } else {
        return None;
    }
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