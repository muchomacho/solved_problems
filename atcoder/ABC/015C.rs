
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let mut poll: Vec<Vec<u8>> = vec![vec![0; k]; n];
    for i in 0..n {
        for j in 0..k {
            poll[i][j] = sc.read()
        }
    }

    let mut bug = false;
    let mut stack: VecDeque<(u8, u8, usize)> = VecDeque::new();
    for &x in poll[0].iter() {
        stack.push_front((x, 0, 0));
    }

    while stack.len() > 0 {
        let (num, temp, index) = stack.pop_front().unwrap();
        if index == n - 1 && num ^ temp == 0 {
            bug = true;
            break;
        } else if index < n - 1 {
            for &x in poll[index + 1].iter() {
                stack.push_front((x, num ^ temp, index + 1));
            }
        }
    }

    if bug {
        println!("Found");
    } else {
            println!("Nothing");
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