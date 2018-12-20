#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::prelude::BufRead;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, BufWriter, Write};
#[allow(unused_imports)]
use std::mem::swap;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    let p = read::<usize>();
    let a = read_vector::<usize>();
    let unique_ids = a.iter().cloned().collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();
    let mut count = unique_ids.iter().map(|i| (*i, 0)).collect::<BTreeMap<_, _>>();

    let all_id_num = unique_ids.len();
    let mut minimum = std::usize::MAX;
    let (mut left, mut right) = (0, 0);
    *count.get_mut(&a[right]).unwrap() += 1;
    let mut id_num = 1;
    loop {
        if id_num < all_id_num {
            while right < p - 1 {
                right += 1;
                if let Some(i) = count.get_mut(&a[right]) {
                    if *i == 0 {
                        id_num += 1;
                    }
                    *i += 1;
                } else {
                    panic!("error");
                }
                if id_num == all_id_num {
                    break;
                }
            }
        }
        if id_num < all_id_num {
            break;
        }
        minimum = min(right - left + 1, minimum);
        if let Some(i) = count.get_mut(&a[left]) {
            if *i == 1 {
                id_num -= 1;
            }
            *i -= 1;
        } else {
            panic!("error");
        }
        left += 1;
    }

    if minimum < std::usize::MAX {
        println!("{}", minimum);
    } else {
        println!("0");
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
    return buf.chars().collect();
}
