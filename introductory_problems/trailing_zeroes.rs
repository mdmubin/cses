fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin
        .read_line(&mut buf)
        .expect("Failed to read line for 'num'.");

    let n = buf
        .trim()
        .parse::<u64>()
        .expect("Failed to parse i32 for 'num'");

    let mut pow: u64 = 5;
    let mut zero_count: u64 = 0;

    while pow <= n {
        zero_count += n / pow;
        pow *= 5;
    }

    print!("{}", zero_count);
}
