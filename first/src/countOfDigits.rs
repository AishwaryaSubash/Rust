pub fn countDigits() {
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read string");
    // let mut n:u32 = s.trim().parse().unwrap();
    let mut count: i32 = 0;
    for c in s.chars() {
        if c == '1' {
            count += 1;
        }
    }
    print!("{}", count);
}
