use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 27;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

const UNVISITED: char = '.';
const QUEEN: char = 'Q';

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_4(& mut board, 0);
    let duration = start.elapsed();

    println!("Execution time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&board);
}


// Try to place a queen in this column.
// Return true if we find a legal board.
fn place_queens_4(board: &mut [[char; NUM_COLS]; NUM_ROWS], col_index: usize) -> bool {
    // Have we placed all the queens? If so, we just return true of false depending on if the board is legal or not.
    if col_index == NUM_ROWS {
        return board_is_a_solution(board, NUM_ROWS as u32);
    }

    // OK, we haven't places all queens yet.
    // Start with checking if the board is legal at this point.
    if !board_is_legal(board){
        return false;
    }

    // Board is still legal. Try to place a queen in this column at each row one at a time.
    for row_index in 0..NUM_ROWS {
        // Place a queen in this row.
        board[row_index][col_index] = QUEEN;

        // Perform a recursive call to place a queen in the next (to the right) column.
        // Did we find a solution in this path?
        if place_queens_4(board, col_index + 1) {
            // Yes! Return true to indicate that we did find a solution.
            return true;
        }

        // No, apparently placing a queen here did not lead to us finding a solution => remove it
        // and try placing it in the next row.
        board[row_index][col_index] = UNVISITED;
    }

    // We did not find any solution on this path => return false.
    false
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
