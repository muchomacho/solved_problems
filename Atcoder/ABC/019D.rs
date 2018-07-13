
use std::io::Write;

fn main() {
    let n: usize = read();
    let mut maximum: usize = 0;
    let mut p: usize = 0;
    for i in 2..(n + 1) {
        println!("? {} {}", 1, i);
        std::io::stdout().flush();
        let res: usize = read();
        if res > maximum {
            maximum = res;
            p = i;
        }
    }
    maximum = 0;
    for i in 1..(n + 1) {
        if i == p {
            continue;
        }
        println!("? {} {}", p, i);
        std::io::stdout().flush();
        let res: usize = read();
        if res > maximum {
            maximum = res;
        }
    }

    println!("! {}", maximum);
}

fn read<T>() -> T where T: std::str::FromStr, T::Err: std::fmt::Debug {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    return buf
        .trim()
        .parse()
        .unwrap();
}
