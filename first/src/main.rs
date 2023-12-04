use std::io;
// use std::collections::HashSet;
mod sum;

fn main() {
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read string");
    let mut m: u32 = s.trim().parse().unwrap();

    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read string");
    let mut n: u32 = t.trim().parse().unwrap();

    let v: Vec<i64> = Vec::new();
}
