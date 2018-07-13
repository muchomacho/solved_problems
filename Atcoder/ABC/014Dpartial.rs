
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
    let q: usize = sc.read();
    let mut count: Vec<(usize, usize)> = (0..n).map(|x| (x, 0)).collect();
    let mut add: Vec<Vec<(usize, usize)>> = (0..n).map(|_| vec![]).collect();
    let mut answer: Vec<usize> = vec![0; q];
    for i in 0..q {
        let start: usize = sc.read();
        let end: usize = sc.read();
        count[start - 1].1 += 1;
        count[end - 1].1 += 1;
        add[start - 1].push((i, end - 1));
        add[end - 1].push((i, start - 1));
    }
    count.sort_by_key(|& x| x.1);
    count.reverse();
    for &(index, c) in count.iter(){
        let mut done = 0;
        for &(i, _point) in add[index].iter(){
            if answer[i] != 0 {
                done += 1;
            }
        }
        if done == c {
            continue;
        }
        let times = bfs(index, n, &connect);
        for &(i, point) in add[index].iter(){
            answer[i] = times[point] + 1;
        }
    }
    for i in answer.iter() {
        println!("{}", i);
    }
}

fn bfs(start: usize, n: usize, connect: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut times: Vec<usize> = vec![0; n];
    let mut queue : Vec<(usize, usize)> = connect[start].iter().map(|x| (*x, 1)).collect();
    let mut used: Vec<bool> = (0..n).map(|_| false).collect();
    used[start] = true;
    while queue.len() > 0 {
        let (point, time) = queue.pop().unwrap();
        times[point] = time;
        for i in connect[point].iter() {
            if ! used[*i] {
                queue.push((*i, time + 1));
                used[*i] = true;
            }
        }
    }
    return times;
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