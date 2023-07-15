fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Failed to read line");
    let num = buf
        .trim()
        .parse::<i32>()
        .expect("Failed to parse i32 for 'i'.");

    let total = num as i64 * (num as i64 + 1) / 2;

    if total & 1 == 1 {
        print!("NO SOLUTION");
    } else {
        println!("YES");

        let mut a1: Vec<i32> = Vec::with_capacity((num / 2) as usize);
        let mut a2: Vec<i32> = Vec::with_capacity((num / 2) as usize);

        let mut i = 1;
        let mut j = num - (num & 1);

        let mut first = true;
        while i < j {
            if first {
                a1.push(i);
                a1.push(j);
            } else {
                a2.push(i);
                a2.push(j);
            }
            first = !first;
            i += 1;
            j -= 1;
        }

        if (num & 1) == 1 {
            a2.push(num);
        }

        println!("{}", a1.len());
        for i in a1 {
            print!("{} ", i)
        }
        println!("\n{}", a2.len());
        for i in a2 {
            print!("{} ", i)
        }
        println!("");
    }
}
