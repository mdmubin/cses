#[allow(unused_parens)]
fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin
        .read_line(&mut buf)
        .expect("Failed to read line for 'n'.");

    let n = buf
        .trim()
        .parse::<u64>()
        .expect("Failed to parse i32 for 'n'");

    let (mut a, mut b);
    for _ in 0..n {
        buf.clear();
        stdin
            .read_line(&mut buf)
            .expect("Failed to read line for a and b.");

        let ab_vals: Vec<i32> = buf
            .trim()
            .split(" ")
            .map(|i| i.parse::<i32>().expect("Failed to parse i32 for a and b"))
            .collect();

        a = std::cmp::min(ab_vals[0], ab_vals[1]);
        b = std::cmp::max(ab_vals[0], ab_vals[1]);

        if ((a + b) % 3 == 0) && (b <= (a * 2)) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
