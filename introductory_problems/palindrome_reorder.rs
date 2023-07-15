use std::collections::LinkedList;

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();

    stdin.read_line(&mut line).expect("Failed to read line.");

    let mut charmap = [0; 26];

    line.chars().for_each(|c| {
        if c.is_alphabetic() {
            charmap[c as usize - 'A' as usize] += 1;
        }
    });

    let mut odd_idx = usize::MAX;

    for i in 0..26 {
        if charmap[i] & 1 == 1 {
            if odd_idx != usize::MAX {
                print!("NO SOLUTION");
                return;
            }
            odd_idx = i;
        }
    }

    let mut res = LinkedList::new();

    if odd_idx != usize::MAX {
        for _ in 0..charmap[odd_idx] {
            res.push_back((odd_idx + 'A' as usize) as u8 as char);
        }
        charmap[odd_idx] = 0;
    }

    for i in 0..26 {
        for _ in 0..(charmap[i] / 2) {
            res.push_back((i + 'A' as usize) as u8 as char);
            res.push_front((i + 'A' as usize) as u8 as char);
        }
    }

    let res = res.iter().collect::<String>();
    print!("{}", res);
}
