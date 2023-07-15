fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    // count variable (ignored)
    stdin.read_line(&mut buf).expect("Failed to read line");

    let n = buf
        .trim()
        .parse::<i32>()
        .expect("Failed to parse int for `n`.");

    if n == 1 {
        print!("{}", n)
    } else if n < 4 {
        print!("NO SOLUTION")
    } else {
        for i in (2..=n).step_by(2) {
            print!("{} ", i)
        }

        for i in (1..=n).step_by(2) {
            print!("{} ", i)
        }
    }
}
