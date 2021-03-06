
fn main() {
    let mut sc = Scanner::new();
    let w:usize = sc.read();
    let n:usize = sc.read();
    let k:usize = sc.read();
    let mut width: Vec<usize> = vec![0; n];
    let mut value: Vec<usize> = vec![0; n];
    for i in 0..n {
        width[i] = sc.read();
        value[i] = sc.read();
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; n]; (w + 1)]; (k + 1)];
    for i in 1..(k + 1) {
        for j in 1..(w + 1) {
            for l in 0..n {
                if l == 0 {
                    if j >= width[l] {
                        dp[i][j][l] = value[l];
                    }
                } else {
                    let mut max = dp[i][j][l - 1];
                    if j >= width[l] {
                        if dp[i - 1][j - width[l]][l - 1] + value[l] >= max {
                            max = dp[i - 1][j - width[l]][l - 1] + value[l];
                        }
                    }
                    dp[i][j][l] = max;
                }
            }
        }
    }
    println!("{}", dp[k][w][n - 1]);
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