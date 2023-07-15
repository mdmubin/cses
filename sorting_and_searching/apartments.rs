// #![allow(unused)]

fn soln(k: i32, desired: &Vec<i32>, apartments: &Vec<i32>) -> u64 {
    let (mut c, mut i, mut j) = (0, 0, 0);
    let mut diff;
    while i < desired.len() && j < apartments.len() {
        diff = desired[i] - apartments[j];
        if diff.abs() <= k {
            c += 1;
            i += 1;
            j += 1;
        } else if diff > 0 {
            j += 1;
        } else {
            i += 1;
        }
    }
    return c;
}

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin
        .read_line(&mut buf)
        .expect("Failed to read line for n.m.k vals");

    let mut it = buf
        .trim()
        .split(" ")
        .map(|i| i.parse::<i32>().expect("err"));

    let _ = it.next(); // unused val, m
    let _ = it.next(); // unused val, n
    let k = it.next().unwrap();

    buf.clear();
    stdin
        .read_line(&mut buf)
        .expect("Failed to read line for apart vals");

    let mut desired = buf
        .trim()
        .split(" ")
        .map(|i| i.parse::<i32>().expect("err"))
        .collect::<Vec<i32>>();
    desired.sort();

    buf.clear();
    stdin
        .read_line(&mut buf)
        .expect("Failed to read line for apart vals");

    let mut apartments = buf
        .trim()
        .split(" ")
        .map(|i| i.parse::<i32>().expect("err"))
        .collect::<Vec<i32>>();
    apartments.sort();

    print!("{}", soln(k, &desired, &apartments));
}
