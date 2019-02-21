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
    let mut adj = vec![HashSet::new(); n];
    for i in 0..n {
        let line = read_vector::<usize>();
        for j in 0..n {
            if line[j] == 1 {
                adj[i].insert(j);
            }
        }
    }
    
    println!("{}", maximal_matching(&adj));
}

fn maximal_matching(adj: &[HashSet<usize>]) -> usize {
    let mut matching = vec![std::usize::MAX; adj.len()];
    let mut matched = 0;
    while let Some(path) = find_augmenting_path(adj, &matching) {        
        for i in 0..(path.len() - 1) {
            if i % 2 == 0 {
                matching[path[i]] = path[i + 1];
                matching[path[i + 1]] = path[i];
            }
        }
        matched += 1;
    }
    matched
}

fn find_augmenting_path(adj: &[HashSet<usize>], matching: &[usize]) -> Option<Vec<usize>> {
    let mut unmarked_edges = adj.clone().to_vec();
    let mut unmarked_vertices = VecDeque::new();
    let mut forest = vec![(std::usize::MAX, std::usize::MAX, std::usize::MAX); adj.len()];

    // add exposed vertex to unmarked_vertices
    // remove matched edge from unmarked edges
    for i in 0..matching.len() {
        if matching[i] == std::usize::MAX {
            forest[i] = (i, i, 0);
            unmarked_vertices.push_back(i);
        } else {
            let j = matching[i];
            unmarked_edges[i].remove(&j);
            unmarked_edges[j].remove(&i);
        }
    }
    
    // each unmarked vertex v with even length from its root
    while unmarked_vertices.len() > 0 {
        let v = unmarked_vertices.pop_front().unwrap();
        // each unmarked edge (v, w)
        for &w in unmarked_edges[v].iter() {
            // if w is matched, add path to tree
            if forest[w].0 == std::usize::MAX {
                let j = matching[w];
                forest[w] = (v, forest[v].1, forest[v].2 + 1);
                forest[j] = (w, forest[v].1, forest[v].2 + 2);
                unmarked_vertices.push_back(j);
            } else {
                // if distance(w, root(w)) is odd, do nothing
                if forest[w].2 % 2 == 1 {
                    continue;
                }
                // if root(v) == root(w), create a blossom and call function recursively
                if forest[w].1 == forest[v].1 {
                    // find blossom
                    let blossom_path = find_blossom(w, v, &forest);
                    let mut in_blossom = vec![false; adj.len()];
                    for &i in blossom_path.iter() {
                        in_blossom[i] = true;
                    }
                    // contract graph and matching by blossom
                    let mut cont_adj = vec![HashSet::new(); adj.len() - blossom_path.len() + 1];
                    let mut cont_matching = vec![std::usize::MAX; adj.len() - blossom_path.len() + 1];
                    let mut cont_to_original = (0..adj.len()).filter(|&i| !in_blossom[i]).collect::<Vec<_>>();
                    let mut original_to_cont = vec![adj.len() - blossom_path.len(); adj.len()];
                    for i in 0..cont_to_original.len() {
                        original_to_cont[cont_to_original[i]] = i;
                    }
                    for i in 0..adj.len() {    
                        for &item in adj[i].iter() {
                            cont_adj[original_to_cont[i]].insert(original_to_cont[item]);
                        }
                        if in_blossom[i] {
                            if matching[i] < std::usize::MAX && !in_blossom[matching[i]] {
                                cont_matching[original_to_cont[i]] = original_to_cont[matching[i]];
                            }
                        } else {
                            if matching[i] < std::usize::MAX {
                                cont_matching[original_to_cont[i]] = original_to_cont[matching[i]];
                            }
                        }
                    }
                    cont_adj.last_mut().unwrap().remove(&(adj.len() - blossom_path.len()));

                    // if augmenting path is found on contracted graph, extend it to augmenting path on original graph
                    if let Some(cont_path) = find_augmenting_path(&cont_adj, &cont_matching) {
                        let mut original_path = vec![];
                        for i in 0..cont_path.len() {
                            if cont_path[i] < cont_to_original.len() {
                                original_path.push(cont_to_original[cont_path[i]]);
                            } else {
                                // recover path so that the original path is still an alternating path
                                let mut prv = cont_to_original[cont_path[i - 1]];
                                let mut nxt = cont_to_original[cont_path[i + 1]];
                                let mut reverse = false;
                                if matching[nxt] < std::usize::MAX && in_blossom[matching[nxt]] {
                                    swap(&mut prv, &mut nxt);
                                    reverse = true;
                                }
                                let mut path = vec![matching[prv]];
                                let root_index = (0..blossom_path.len()).filter(|&i| blossom_path[i] == matching[prv]).next().unwrap();
                                let mut index = root_index;
                                // find alternating path in blossom with even length
                                // possible number of path is 2-way
                                for dir in [1, -1].into_iter() {
                                    for _ in 0..(blossom_path.len() - 1) {
                                        index = (blossom_path.len() as i32 + index as i32 + dir) as usize % blossom_path.len();
                                        path.push(blossom_path[index]);
                                        if adj[blossom_path[index]].contains(&nxt) {
                                            break;
                                        }
                                    }
                                    if path.len() % 2 == 0 {
                                        break;
                                    }
                                    path = vec![matching[prv]];
                                    index = root_index;
                                }
                                if reverse {
                                    path.reverse();
                                }
                                original_path.append(&mut path);
                            }
                        }
                        return Some(original_path);
                    }
                } else {
                    // augmenting path is found
                    let mut path = vec![v];
                    let mut x = v;
                    while x != forest[x].0 {
                        x = forest[x].0;
                        path.push(x);
                    }
                    path.reverse();
                    path.push(w);
                    x = w;
                    while x != forest[x].0 {
                        x = forest[x].0;
                        path.push(x);
                    }
                    return Some(path);
                }
            }
        }
        // remove checked edges from unmarked edges
        let remove = unmarked_edges[v].iter().map(|i| *i).collect::<Vec<_>>();
        for w in remove {
            unmarked_edges[w].remove(&v);
        }
    }
    // no path is found
    None
}

fn find_blossom(mut x1: usize, mut x2: usize, forest: &[(usize, usize, usize)]) -> Vec<usize> {
    // returned path represents circuit
    let mut x1_path = vec![];
    let mut x2_path = vec![];
    if forest[x1].2 < forest[x2].2 {
        swap(&mut x1, &mut x2);
    }

    while forest[x1].2 > forest[x2].2 {
        x1_path.push(x1);
        x1 = forest[x1].0;
    }

    // the root of blossom is LCA of x1 and x2 
    while forest[x1].0 != forest[x2].0 {
        x1_path.push(x1);
        x2_path.push(x2);
        x1 = forest[x1].0;
        x2 = forest[x2].0;
    }
    x1_path.push(forest[x1].0);
    x2_path.reverse();
    x1_path.append(&mut x2_path);
    
    x1_path
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
