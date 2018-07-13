use std::collections::HashSet;

fn main() {
    let n: usize = read();
    let nums: Vec<usize> = read_vector();
    let mut set = HashSet::new();
    let mut count = 0;
    for mut num in nums.into_iter() {
        loop {
            if num % 2 == 0 {
                num /= 2;
            } else {
                break;
            }
        }

        if !set.contains(&num) {
            set.insert(num);
            count += 1;
        }
    }

    println!("{}", count);
}

fn read<T>() -> T where T: std::str::FromStr, T::Err: std::fmt::Debug {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    return buf
        .trim()
        .parse()
        .unwrap();
}

fn read_vector<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    return buf
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
}
