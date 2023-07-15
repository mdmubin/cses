use std::collections::HashSet;

fn main() {
    let stdin = std::io::stdin();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer.clear();
    stdin
        .read_line(&mut buffer)
        .expect("Failed to read Integer values.");

    let hm: HashSet<i32> = buffer
        .split(" ")
        .map(|x| x.trim().parse::<i32>().expect("Not an integer"))
        .collect();

    print!("{}", hm.len())
}
