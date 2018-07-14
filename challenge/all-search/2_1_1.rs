
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let numbers: Vec<usize> = (0..n).map(|_| sc.read()).collect();
    let k: usize = sc.read();

    if feasible(numbers, k) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn feasible(vec: Vec<usize>, sum: usize) -> bool {
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    stack.push_front((0, 0));
    while stack.len() != 0 {
        let (index, val) = stack.pop_front().unwrap();
        if val + vec[index] == sum {
            return true;
        } else if index == vec.len() - 1 {
            continue;
        } else if val + vec[index] < sum {
            stack.push_front((index + 1, val + vec[index]));
        }
        stack.push_front((index + 1, val));
    }
    return false;
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