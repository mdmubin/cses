fn generate(vals: &mut Vec<u16>, depth: u16, max_depth: u16) {
    if depth == max_depth {
        return;
    }
    let mut cur = Vec::with_capacity(vals.len());
    for i in vals.iter().rev() {
        cur.push(*i + (2 << depth));
    }
    vals.append(&mut cur);
    generate(vals, depth + 1, max_depth);
}

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Failed to read line");

    let n = buf
        .trim()
        .parse::<u32>()
        .expect("Failed to parse u16 for n");

    let mut res = Vec::with_capacity(usize::pow(2, n));

    res.push(0);
    res.push(1);

    generate(&mut res, 0, n as u16 - 1);
    for i in res {
        println!("{:0>1$b}", i, n as usize);
    }
}
