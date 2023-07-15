fn isvalid(board: &[[i32; 8]; 8], row: usize, col: usize) -> bool {
    if board[row][col] == 0 {
        // check vertically
        for i in 0..row {
            if board[i][col] == 2 {
                return false;
            }
        }

        // check left diagonal
        let (mut x, mut y) = (row, col);
        loop {
            if board[x][y] == 2 {
                return false;
            }
            if x == 0 || y == 0 {
                break;
            }
            x -= 1;
            y -= 1;
        }

        // check right diagonal
        let (mut x, mut y) = (row, col);
        loop {
            if board[x][y] == 2 {
                return false;
            }
            if x == 0 || y == 7 {
                break;
            }
            x -= 1;
            y += 1
        }

        return true;
    }

    return false;
}

fn valid_pos_count(board: &mut [[i32; 8]; 8], depth: usize) -> i32 {
    if depth == 8 {
        return 1;
    }

    let mut count = 0;

    for i in 0..8 {
        // if current position is valid queen position
        if isvalid(board, depth, i) {
            // place queen in current position
            board[depth][i] = 2;
            // count number of valid positions after this queen setup
            count += valid_pos_count(board, depth + 1);
            // reset changed board
            board[depth][i] = 0;
        }
    }

    return count;
}

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    let mut board = [[0; 8]; 8];

    for i in 0..8 {
        stdin.read_line(&mut buf).expect("Failed to read line");

        for j in buf.trim().char_indices() {
            board[i][j.0] = match j.1 {
                '.' => 0,
                '*' => 1,
                x => panic!("Unexpected character: '{}'", x),
            }
        }

        buf.clear();
    }

    println!("{}", valid_pos_count(&mut board, 0))
}
