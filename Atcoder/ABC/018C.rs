
use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let r: usize = sc.read();
    let c: usize = sc.read();
    let k: usize = sc.read();

    if r - k < k - 1 || c - k < k - 1 {
        println!("{}", 0);
        return;
    }

    let mut grid: Vec<Vec<i32>> = vec![vec![0; c]; r];
    for i in 0..r {
        let line: String = sc.read();
        for (j, ch) in line.chars().enumerate() {
            if ch == 'x' {
                for l in 0..(2 * k - 1) {
                    let row = (i + l + 1) as i32 - k as i32;
                    if row >= 0 && row <= (r - 1) as i32 {
                        let row = row as usize;
                        let range;
                        if l <= k - 1 {
                            range = l;
                        } else {
                            range = 2 * k - 2 - l;
                        }
                        let left = cmp::max(j as i32 - range as i32, 0) as usize;
                        let right = j + range + 1;
                        grid[row][left] += 1;
                        if right < c {
                            grid[row][right] -= 1;
                        }
                    }
                }
            }
        }
    }

    for i in 0..r {
        for j in 1..c {
            grid[i][j] += grid[i][j - 1];
        }
    }

    let mut count = 0;
    for i in (k - 1)..(r - k + 1) {
        for j in (k - 1)..(c - k + 1) {
            if grid[i][j] == 0 {
                count += 1;
            }
        }
    }

    println!("{}", count);

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