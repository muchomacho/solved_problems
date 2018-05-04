
fn main() {
    let mut sc = Scanner::new();
    let start: Point = Point::new(sc.read(), sc.read());
    let end: Point = Point::new(sc.read(), sc.read());
    let cut_line: Line = Line::new(&start, &end);
    let n: usize = sc.read();
    let mut points: Vec<Point> = vec![];
    for _ in 0..n {
        points.push(Point::new(sc.read(), sc.read()));
    }
    let mut cross_points = 0;
    for i in 0..n {
        let line: Line;
        if i == n - 1 {
            line = Line::new(&points[i], &points[0]);
        } else {
            line = Line::new(&points[i], &points[i + 1]);
        }
        if let Some(point) = line.cross(&cut_line) {
            if (line.start.0 - point.0) * (line.end.0 - point.0) <= 0.0 &&
                (line.start.1 - point.1) * (line.end.1 - point.1) <= 0.0 &&
                (cut_line.start.0 - point.0) * (cut_line.end.0 - point.0) <= 0.0 &&
                (cut_line.start.1 - point.1) * (cut_line.end.1 - point.1) <= 0.0 {

                cross_points += 1;
            }
        }
    }

    println!("{}", 1 + cross_points / 2);

}

struct Point (f32, f32);
impl Point {
    fn new(a: f32, b: f32) -> Point {
        Point(a, b)
    }
}

struct Line<'a> {
    start: &'a Point,
    end: &'a Point,
}

impl<'a> Line<'a> {
    fn new(a: &'a Point, b: &'a Point) -> Line<'a> {
        Line {start: a, end: b}
    }

    fn cross(&self, line: &Line) -> Option<Point> {
        if let Some(my_grad) = self.gradient() {
            if let Some(grad) = line.gradient() {
                let cross_x = (my_grad * self.start.0 - grad * line.start.0 - self.start.1 + line.start.1) / (my_grad - grad);
                let cross_y = my_grad * (cross_x - self.start.0) + self.start.1;
                Some(Point::new(cross_x, cross_y))
            }
                else {
                    Some(Point::new(line.start.0, my_grad * (line.start.0 - self.start.0) + self.start.1))
                }
        } else {
            if let Some(grad) = line.gradient() {
                Some(Point::new(self.start.0, grad * (self.start.0 - line.start.0) + line.start.1))
            }
                else {
                    None
                }
        }
    }

    fn gradient(&self) -> Option<f32> {
        if self.start.0 == self.end.0 {
            None
        } else {
            Some((self.start.1 - self.end.1) / (self.start.0 - self.end.0))
        }
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