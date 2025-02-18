use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

const UNVISITED: char = '.';
const QUEEN: char = 'Q';

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    //let success = place_queens_1(&mut board, 0, 0);
    let success = place_queens_2(&mut board, 0, 0, 0);
    //let success = place_queens_3(& mut board);
    let duration = start.elapsed();

    println!("Execution time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&board);
}

fn place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], row_index: usize, col_index: usize) -> bool {
    // Have we examined all squares? I.e. is row_index >= the number of rows.
    if row_index >= NUM_ROWS {
        return board_is_a_solution(board, NUM_ROWS as u32);
    }

    // OK, we're not finished yet.
    // Handle the two cases:
    // 1. We do not place a Queen on this square
    // or
    // 2. We do place a queen on this square.
    let (next_row_index, next_col_index) = next_square(row_index, col_index);

    // First try without placing a queen on this square. If that is successful, we are done.
    if place_queens_1(board, next_row_index, next_col_index) {
        return true;
    }

    // OK, we did not find a solution when this square was left empty. Try placing a queen here.
    board[row_index][col_index] = QUEEN;
    if place_queens_1(board, next_row_index, next_col_index) {
        return true;
    }

    // That did not work either => this path was a dead end.
    // Remove the queen and backtrack.
    board[row_index][col_index] = UNVISITED;
    false
}

fn next_square(row_index: usize, col_index: usize) -> (usize, usize) {
    if col_index >= NUM_COLS - 1{
        return (row_index + 1, 0);
    }
    (row_index, col_index + 1)
}


// Return true if the board is legal and a solution.
fn board_is_a_solution(board: &[[char; NUM_COLS]; NUM_ROWS], num_queens: u32) -> bool {
    board_is_legal(board) && number_of_queens_in_board(board) == num_queens
}

// Return true if the board is legal.
fn board_is_legal(board: &[[char; NUM_COLS]; NUM_ROWS]) -> bool {
    for i in 0..NUM_ROWS {
        let i = i as i32;
        if !series_is_legal(board, i, 0, 0, 1) || // row
            !series_is_legal(board, 0, i, 1, 0) || // column

            !series_is_legal(board, 0, i, 1, 1) || // down-right from top row
            !series_is_legal(board, 0, i, 1, -1) || // down-left from top row

            !series_is_legal(board, i, 0, 1, 1) || // down-right from leftmost col

            !series_is_legal(board, i, INUM_COLS - i, 1, -1) // down-left from rightmost column.
        {
            return false;
        }
    }
    // If we end up here, all checks passed.
    true
}

// Return true if this series of squares contains at most one queen.
fn series_is_legal(
    board: &[[char; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut number_of_queens_found = 0;
    let mut row = r0;
    let mut col = c0;
    while row >= 0 && row < INUM_ROWS && col >= 0 && col < INUM_COLS {
        if board[row as usize][col as usize] == QUEEN {
            number_of_queens_found += 1;
        }
        row += dr;
        col += dc;
    }

    number_of_queens_found <= 1
}

fn number_of_queens_in_board(board: &[[char; NUM_COLS]; NUM_ROWS]) -> u32 {
    let mut number_of_queens:u32 = 0;
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            if  board[row][col]  == QUEEN {
                number_of_queens += 1;
            }
        }
    }
    number_of_queens
}

fn dump_board(board: &[[char; NUM_COLS]; NUM_ROWS]) {
    for row in board.iter() {
        for &col_value in row.iter() {
            print!("{square_value:>2}", square_value = col_value);
        }
        println!();
    }
}

fn place_queens_2(board: &mut [[char; NUM_COLS]; NUM_ROWS], row_index: usize, col_index: usize, num_placed: usize) -> bool {
    // Have we placed all the queens? In that case, check if the board is a solution.
    if num_placed == NUM_ROWS {
        return board_is_a_solution(board, NUM_ROWS as u32);
    }

    // Have we examined all squares? I.e. is row_index >= the number of rows.
    if row_index >= NUM_ROWS {
        return board_is_a_solution(board, NUM_ROWS as u32);
    }

    // OK, we're not finished yet.
    // Handle the two cases:
    // 1. We do not place a Queen on this square
    // or
    // 2. We do place a queen on this square.
    let (next_row_index, next_col_index) = next_square(row_index, col_index);

    // First try without placing a queen on this square. If that is successful, we are done.
    if place_queens_2(board, next_row_index, next_col_index, num_placed) {
        return true;
    }

    // OK, we did not find a solution when this square was left empty. Try placing a queen here.
    board[row_index][col_index] = QUEEN;
    if place_queens_2(board, next_row_index, next_col_index, num_placed + 1) {
        return true;
    }

    // That did not work either => this path was a dead end.
    // Remove the queen and backtrack.
    board[row_index][col_index] = UNVISITED;
    false
}