
fn main() {
    let mut sc = Scanner::new();
    let param: Vec<usize> = (0..5).map(|_| sc.read()).collect();
    let (n, m, p, q, r) = (param[0], param[1], param[2], param[3], param[4]);
    let mut choco: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..r {
        choco.push((sc.read::<usize>() - 1, sc.read::<usize>() - 1, sc.read()));
    }
    let combi = combination(&(0..n).collect(), p).unwrap();
    let mut maximum = 0;
    for vec in combi.iter() {
        let mut woman_used: Vec<bool> = vec![false; n];
        for &i in vec.iter() {
            woman_used[i] = true;
        }
        let mut man_score: Vec<usize> = vec![0; m];
        for &(i, j, score) in choco.iter() {
            if woman_used[i] {
                man_score[j] += score;
            }
        }
        man_score.sort();
        let mut sum = 0;
        for i in 0..q {
            sum += man_score[m - 1 - i];
        }
        if sum > maximum {
            maximum = sum;
        }
    }

    println!("{}", maximum);
}

fn combination(vector: &Vec<usize>, n: usize) -> Option<Vec<Vec<usize>>>{
    if n == 0 {
        return Some(vec![vec![]]);
    } else if vector.len() < n {
        return None;
    } else if vector.len() == n {
        let copy_vector = vector.clone();
        return Some(vec![copy_vector]);
    } else {
        let mut copy_vector = vector.clone();
        let last = copy_vector.pop().unwrap();
        let mut include: Vec<Vec<usize>> = combination(&copy_vector, n - 1).unwrap();
        for combi in include.iter_mut() {
            combi.push(last);
        }
        let mut exclude: Vec<Vec<usize>> = combination(&copy_vector, n).unwrap();
        include.append(&mut exclude);
        return Some(include);
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