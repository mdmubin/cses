fn generate(all: &mut Vec<String>, cur: &mut Vec<u8>, charmap: &mut [i32; 26], l: usize) {
    if cur.len() == l {
        let s = String::from_utf8(cur.to_vec()).expect("Failed to convert to string");
        all.push(s);
        return;
    }

    for i in 0..26 {
        if charmap[i] != 0 {
            charmap[i] -= 1;
            cur.push(i as u8 + 'a' as u8);
            generate(all, cur, charmap, l);
            cur.pop();
            charmap[i] += 1;
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    let mut charmap = [0; 26];
    let line = buf.trim().as_bytes();

    for c in line {
        charmap[(*c - 'a' as u8) as usize] += 1;
    }

    let mut all = Vec::with_capacity(usize::pow(2, line.len() as u32));
    let mut cur = Vec::with_capacity(line.len());

    generate(&mut all, &mut cur, &mut charmap, line.len());

    println!("{}", all.len());
    for s in all {
        println!("{}", s);
    }
}
