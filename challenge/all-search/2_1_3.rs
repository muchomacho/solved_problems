
use std::collections::VecDeque;

static SEARCH: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph: Vec<Vec<bool>> = vec![vec![false; m + 2]; n + 2];
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);
    for i in 0..n {
        let mut line: String = sc.read();
        for (j, chr) in line.chars().enumerate() {
            if chr == 'S' {
                start = (i + 1, j + 1);
            } else if chr == 'G' {
                goal = (i + 1, j + 1);
            }
            graph[i + 1][j + 1] = chr != '#';
        }
    }

    if start == (0, 0) || goal == (0, 0) {
        panic!("start or goal is not defined.");
    }
    println!("{}", bfs(graph, start, goal));

}

fn bfs(mut graph: Vec<Vec<bool>>, start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut cost: usize = graph.len() * graph[0].len();
    queue.push_front((start, 0));
    while queue.len() != 0 {
        let ((x, y), t) = queue.pop_back().unwrap();
        if (x, y) == goal {
            cost = t;
            break;
        }
        for &(row, column) in SEARCH.iter() {
            let (new_x, new_y) = ((x as i32 + row) as usize, (y as i32 + column) as usize);
            if graph[new_x][new_y] {
                graph[new_x][new_y] = false;
                queue.push_front(((new_x, new_y), t + 1));
            }
        }
    }

    if cost == graph.len() * graph[0].len() {
        panic!("cannot reach goal");
    }

    return cost;
}


struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024 * 10], small_cache: vec![0; 1024] }
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