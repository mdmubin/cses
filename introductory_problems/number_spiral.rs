fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin
        .read_line(&mut buf)
        .expect("Failed to read line for `n`.");

    let n = buf
        .trim()
        .parse::<i32>()
        .expect("Failed to parse int for `n`.");

    for _ in 0..n {
        buf.clear();
        stdin
            .read_line(&mut buf)
            .expect("Failed to read line for `rxc`.");

        let mut rxc = buf
            .trim()
            .split(' ')
            .map(|i| i.parse::<i128>().expect("Failed to parse int for `rxc`."));

        let r = rxc.next().unwrap();
        let c = rxc.next().unwrap();

        let val = if r == c {
            // diagonal value = n^2 - n + 1
            r * r - r + 1
        } else if r > c {
            let diag = r * r - r + 1;
            if r & 1 == 1 {
                diag - (r - c)
            } else {
                diag + (r - c)
            }
        } else {
            let diag = c * c - c + 1;
            if c & 1 == 1 {
                diag - (r - c)
            } else {
                diag + (r - c)
            }
        };

        println!("{}", val)
    }
}
