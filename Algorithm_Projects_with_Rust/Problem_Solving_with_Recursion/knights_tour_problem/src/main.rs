use std::time::{Instant};

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
//const INUM_ROWS: i32 = NUM_ROWS as i32;
//const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn main() {
    // Initialize the vector of move offsets.
    let offsets = [
        [-2, -1],
        [-1, -2],
        [ 2, -1],
        [ 1, -2],
        [-2,  1],
        [-1,  2],
        [ 2,  1],
        [ 1,  2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&board);
}

fn dump_board(board: &[[i32; NUM_COLS]; NUM_ROWS]) {
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            print!("{number:0>2}", number=board[row][col]);
            print!(" ");
        }
        println!();
    }
}

fn find_tour(board: &mut [[i32; NUM_COLS]; NUM_ROWS], offsets: &[[i32; 2]; 8], cur_row: usize, cur_col: usize,
             num_visited: usize) -> bool {
    //-----------------------------------------------------------------
    // Have we visited all squares?
    //-----------------------------------------------------------------
    if num_visited == NUM_ROWS * NUM_COLS {
        // If we are OK with an open tour, we are done.
        // We are also done if we should have a closed tour and can move to the starting square.
        if !REQUIRE_CLOSED_TOUR || can_move_to(0, 0, cur_row, cur_col, offsets) {
            // Mark this position on the board as visited.
            board[cur_row][cur_col] = num_visited as i32;
            return true;
        }
        // No luck finding a solution on this path :(
        return false;
    }

    //-----------------------------------------------------------------
    // OK, we haven't visited all squares yet.
    //-----------------------------------------------------------------

    // Mark this position on the board as visited.
    board[cur_row][cur_col] = num_visited as i32;

    // Go through all valid moved from here.
    let new_num_visited = num_visited + 1;
    for [row, col] in generate_valid_moves(cur_row, cur_col, offsets) {
        // We haven't visited this square before, have we? If so, don't go there.
        if board[row][col] == UNVISITED {
            // OK, we haven't visited that square. Let's go there!
            if find_tour(board, offsets, row, col, new_num_visited) {
                return true;
        }
    }
}

    // OK, we did not find a solution using this path => mark this square as unvisited...
    board[cur_row][cur_col] = UNVISITED;

    //...and return false to indicate the failure to find a solution using path.
    false
}


fn can_move_to(wanted_row: usize, wanted_col: usize, cur_row: usize, cur_col: usize, offsets: &[[i32; 2]; 8]) -> bool {
    for [row, col] in generate_valid_moves(cur_row, cur_col, offsets) {
        if row == wanted_row && col == wanted_col {
            return true;
        }
    }

    false
}

fn generate_valid_moves( cur_row: usize, cur_col: usize, offsets: &[[i32; 2]; 8]) -> Vec<[usize; 2]> {
    let mut valid_moves: Vec<[usize; 2]> = Vec::new();

    for offset in offsets {
        let next_row = cur_row as i32 + offset[0];
        let next_col = cur_col as i32 + offset[1];
        if next_row >= 0 && next_row < NUM_ROWS as i32 && next_col >= 0 && next_col < NUM_COLS as i32 {
            valid_moves.push([next_row as usize, next_col as usize]);
        }
    }

    valid_moves
}