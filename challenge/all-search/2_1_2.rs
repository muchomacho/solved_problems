use std::collections::VecDeque;

static SEARCH: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, 1),
    (1, 1), (1, 0), (1, -1), (0, -1)];

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph: Vec<Vec<bool>> = vec![vec![false; m + 2]; n + 2];
    for i in 1..(n + 1) {
        let mut line: String = sc.read();
        for (j, pool) in line.chars().enumerate() {
            graph[i][j + 1] = pool == 'W';
        }
    }

    println!("{}", bfs(graph));

}

fn bfs(graph: Vec<Vec<bool>>) -> usize {
    let mut used: Vec<Vec<bool>> = vec![vec![false; graph[0].len()]; graph.len()];
    let mut num: usize = 0;
    for i in 1..(graph.len() - 1) {
        for j in 1..(graph[0].len() - 1) {
            if !graph[i][j] || used[i][j] {
                continue;
            }
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_front((i, j));
            while queue.len() != 0 {
                let (s, t) = queue.pop_back().unwrap();
                for &(row, column) in SEARCH.iter() {
                    let (s, t) = ((s as i32 + row) as usize, (t as i32 + column) as usize);
                    if graph[s][t] && !used[s][t] {
                        queue.push_front((s, t));
                        used[s][t] = true;
                    }
                }
            }
            num += 1;
        }
    }

    num
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