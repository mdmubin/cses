fn tower_of_hanoi(moves: &mut Vec<(u8, u8)>, depth: i32, from: u8, to: u8, other: u8) {
    if depth == 1 {
        moves.push((from, to));
        return;
    }
    tower_of_hanoi(moves, depth - 1, from, other, to);
    moves.push((from, to));
    tower_of_hanoi(moves, depth - 1, other, to, from);
}

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Failed to read line");

    let n = buf
        .trim()
        .parse::<i32>()
        .expect("Failed to parse u16 for n");

    let mut moves: Vec<(u8, u8)> = Vec::with_capacity(usize::pow(2, n as u32));
    tower_of_hanoi(&mut moves, n, 1, 3, 2);

    println!("{}", moves.len());
    for i in moves {
        println!("{} {}", i.0, i.1);
    }
}
