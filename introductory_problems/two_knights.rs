fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Failed to read line");
    let num = buf
        .trim()
        .parse::<i64>()
        .expect("Failed to parse i32 for 'i'.");

    let (mut sqc, mut all, mut inv);
    for i in 1..=num {
        sqc = i.pow(2);
        all = (sqc * (sqc - 1)) / 2;
        inv = ((i - 1) * (i - 2)) * 2 * 2;
        println!("{}", all - inv)
    }
}
