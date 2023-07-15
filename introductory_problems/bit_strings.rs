const MODULO: u32 = 1_000_000_007;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin
        .read_line(&mut buf)
        .expect("Failed to read line for 'num'.");

    let n = buf
        .trim()
        .parse::<u32>()
        .expect("Failed to parse i32 for 'num'");

    let mut res = 1;
    for _ in 0..n {
        res *= 2;
        res %= MODULO;
    }
    print!("{}", res % MODULO);
}
