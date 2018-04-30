
use std::collections::VecDeque;
use std::mem;

fn main(){
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut connect: Vec<Vec<usize>> = (0..n).map(|_| vec![]).collect();
    for _ in 0..(n - 1) {
        let a: usize = sc.read();
        let b: usize = sc.read();
        connect[a - 1].push(b - 1);
        connect[b - 1].push(a - 1);
    }

    let root = 0;
    let log_n = (n as f64).log2() as usize;
    let nodes = bfs(root, n, &connect);
    let mut table: Vec<Vec<usize>> = vec![vec![n; log_n + 1]; n];
    for j in 0..(log_n + 1) {
        for i in 1..n {
            if j == 0 {
                table[i][j] = nodes[i].parent;
            }
                else if table[i][j - 1] != n {
                    table[i][j] = table[table[i][j - 1]][j - 1];
                }
        }
    }

    let q: usize = sc.read();
    for _ in 0..q {
        let mut c1: usize = sc.read();
        let mut c2: usize = sc.read();
        c1 -= 1;
        c2 -= 1;

        if nodes[c1].depth > nodes[c2].depth {
            mem::swap(&mut c1, &mut c2);
        }
        let mut index = log_n;
        let mut up_c2 = c2;
        loop {
            if table[up_c2][index] != n {
                if nodes[table[up_c2][index]].depth >= nodes[c1].depth {
                    up_c2 = table[up_c2][index];
                }
            }
            if nodes[up_c2].depth == nodes[c1].depth {
                break;
            }
            index -= 1;
        }

        if c1 == up_c2 {
            println!("{}", nodes[c2].depth - nodes[c1].depth + 1);
        }
            else {
                let lca_depth;
                let mut index = log_n;
                let mut up_c1 = c1;
                loop {
                    if table[up_c1][index] != table[up_c2][index] {
                        up_c1 = table[up_c1][index];
                        up_c2 = table[up_c2][index];

                    }
                    if index == 0 {
                        break;
                    }
                    index -= 1;
                }
                lca_depth = nodes[up_c1].depth - 1;
                println!("{}", nodes[c2].depth + nodes[c1].depth - 2 * lca_depth + 1);
            }
    }

}

fn bfs(start: usize, n: usize, connect: &Vec<Vec<usize>>) -> Vec<Node> {
    let mut nodes: Vec<Node> = (0..n).map(|_| Node::new()).collect();
    let mut queue : VecDeque<(usize, usize)> = VecDeque::new();
    let mut used: Vec<bool> = (0..n).map(|_| false).collect();
    queue.push_back((start, 0));
    used[start] = true;
    while queue.len() > 0 {
        let (point, depth) = queue.pop_front().unwrap();
        for &i in connect[point].iter() {
            if ! used[i] {
                queue.push_back((i, depth + 1));
                used[i] = true;
                nodes[i].depth = depth + 1;
                nodes[i].parent = point;
            }
        }
    }
    return nodes;
}

struct Node {
    parent: usize,
    depth: usize,
}

impl Node {
    fn new() -> Node {
        Node { parent: 0, depth: 0 }
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }
    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}