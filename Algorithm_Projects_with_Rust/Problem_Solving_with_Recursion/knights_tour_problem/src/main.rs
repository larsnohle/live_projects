use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn main() {
    // Initialize the vector of move offsets.
    let offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
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
            print!("{number:0>2}", number = board[row][col]);
            print!(" ");
        }
        println!();
    }
}

fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &[[i32; 2]; 8],
    cur_row: usize,
    cur_col: usize,
    num_visited: usize,
) -> bool {
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

    // Go through all valid moved from here. The generate_valid_moves() will sort the vector it
    // returns according to Warnsdorf's rule so we just need to traverse it in the order we
    // receive it.
    let new_num_visited = num_visited + 1;
    for [row, col] in generate_valid_moves(cur_row, cur_col, offsets, board) {
        // Let's go there!
        if find_tour(board, offsets, row, col, new_num_visited) {
            return true;
        }
    }

    // OK, we did not find a solution using this path => mark this square as unvisited...
    board[cur_row][cur_col] = UNVISITED;

    //...and return false to indicate the failure to find a solution using path.
    false
}

fn can_move_to(
    wanted_row: usize,
    wanted_col: usize,
    cur_row: usize,
    cur_col: usize,
    offsets: &[[i32; 2]; 8],
) -> bool {
    for offset in offsets {
        let next_row = cur_row as i32 + offset[0];
        let next_col = cur_col as i32 + offset[1];
        if next_row >= 0
            && next_row < NUM_ROWS as i32
            && next_col >= 0
            && next_col < NUM_COLS as i32
            && next_row == wanted_row as i32
            && next_col == wanted_col as i32
        {
            return true;
        }
    }
    false
}

// board[cur_row][cur_col] is assumed to be visited.
fn generate_valid_moves(
    cur_row: usize,
    cur_col: usize,
    offsets: &[[i32; 2]; 8],
    board: &[[i32; NUM_COLS]; NUM_ROWS],
) -> Vec<[usize; 2]> {
    let mut valid_moves: Vec<[usize; 2]> = Vec::new();

    for offset in offsets {
        let next_row = cur_row as i32 + offset[0];
        let next_col = cur_col as i32 + offset[1];
        if next_row >= 0
            && next_row < NUM_ROWS as i32
            && next_col >= 0
            && next_col < NUM_COLS as i32
            && board[next_row as usize][next_col as usize] == UNVISITED
        {
            valid_moves.push([next_row as usize, next_col as usize]);
        }
    }

    // Sort vector in ascending order based on the number of moves from the elements.
    // This is so that we can apply Warnsdorf's rule.
    valid_moves.sort_unstable_by(|[a_row, a_col], [b_row, b_col]| {
        get_number_of_valid_moves_from_square(*a_row, *a_col, offsets, board).cmp(
            &get_number_of_valid_moves_from_square(*b_row, *b_col, offsets, board),
        )
    });

    valid_moves
}

fn get_number_of_valid_moves_from_square(
    row: usize,
    col: usize,
    offsets: &[[i32; 2]; 8],
    board: &[[i32; NUM_COLS]; NUM_ROWS],
) -> usize {
    let mut number_of_valid_moves: usize = 0;
    for offset in offsets {
        let next_row = row as i32 + offset[0];
        let next_col = col as i32 + offset[1];
        if next_row >= 0
            && next_row < NUM_ROWS as i32
            && next_col >= 0
            && next_col < NUM_COLS as i32
            && board[next_row as usize][next_col as usize] == UNVISITED
        {
            number_of_valid_moves += 1;
        }
    }
    number_of_valid_moves
}
