
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let param: Vec<usize> = (0..5).map(|_| sc.read()).collect();
    let (n, m, p, q, r) = (param[0], param[1], param[2], param[3], param[4]);
    let mut choco: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..r {
        choco.push((sc.read::<usize>() - 1, sc.read::<usize>() - 1, sc.read()));
    }
    let mut used: Vec<bool> = vec![false; n + m];
    let mut stack: VecDeque<(usize, usize, usize, usize, Option<usize>, Option<usize>)> = VecDeque::new();
    let mut maximum: usize = 0;
    stack.push_front((0, 0, 0, 0, None, None));
    while stack.len() > 0 {
        let (index, sum, women, men, del_woman, del_man) = stack.pop_front().unwrap();
        if women > p || men > q {
            continue;
        }

        if let Some(i) = del_woman {
            used[i] = false;
        }
        if let Some(i) = del_man {
            used[n + i] = false;
        }

        if index == r {
            if sum > maximum {
                maximum = sum;
            }
            continue;
        }

        let (woman, man, score) = choco[index];
        let (mut new_women, mut new_men) = (women, men);
        let mut del_woman: Option<usize> = None;
        let mut del_man: Option<usize> = None;
        if !used[woman] {
            new_women += 1;
            used[woman] = true;
            del_woman = Some(woman)
        }
        if !used[n + man] {
            new_men += 1;
            used[n + man] = true;
            del_man = Some(man);
        }

        stack.push_front((index + 1, sum, women, men, del_woman, del_man));
        stack.push_front((index + 1, sum + score, new_women, new_men, None, None));
    }

    println!("{}", maximum);
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