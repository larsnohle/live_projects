const NUM_DISKS: usize = 3;

fn main() {
    // Make three posts with NUM_DISKS entries, all set to 0.
    let mut posts = [[0; NUM_DISKS]; 3];

    // Put the disks on the first post in order, smallest first (on top).
    for i in 0..NUM_DISKS {
        posts[0][i] = i + 1;
    }

    // Draw the initial setup.
    draw_posts(&mut posts);

    // Move the disks.
    move_disks(&mut posts, NUM_DISKS, 0, 1, 2);
    println!("Ok");
}


// Draw the posts by showing the size of the disk at each level.
fn draw_posts(posts: &mut [[usize; NUM_DISKS]; 3]) {
    for row_index in 0..NUM_DISKS {
        for post_index in 0..posts.len() {
            print!("{:?} ", posts[post_index][row_index]);
        }
        println!();
    }
    println!("-----");
}

// Move the disks from from_post to to_post
// using temp_post as temporary storage.
fn move_disks(posts: &mut [[usize; NUM_DISKS]; 3],
              num_to_move: usize, from_post: usize, to_post: usize, temp_post: usize) {
    // Base case: nothing to move.
    if num_to_move == 0 {
        return;
    }

    // Move all but one of the disks from the from post to the temp post. Use the to post as tmp
    // post.
    move_disks(posts, num_to_move - 1, from_post, temp_post, to_post);

    // Now we can move the last disk from the from post to the to post.
    move_disk(posts, from_post, to_post);

    // Move the disks we moved to the temp disk to the to post.
    move_disks(posts, num_to_move - 1, temp_post, to_post, from_post);
}

// Move one disk from from_post to to_post.
fn move_disk(posts: &mut [[usize; NUM_DISKS]; 3], from_post: usize, to_post: usize) {
    // Find the index of the disk we want to move.
    // Remember that posts grow downwards.
    let mut index_of_topmost_disk_in_from_post: usize = 0;
    let mut disk_to_move_found = false;
    for (index, &disk_value) in posts[from_post].iter().enumerate() {
        if disk_value != 0 {
            index_of_topmost_disk_in_from_post = index;
            disk_to_move_found = true;
            break;
        }
    }
    // Sanity check.
    if !disk_to_move_found {
        panic!("Did not find disk to move");
    }


    // Find the first empty position in the post we want to move to.
    let mut index_of_first_empty_position_in_to_post: usize = 0;
    let mut empty_position_found : bool = false;
    for (index, &value) in posts[to_post].iter().rev().enumerate() {
        if value == 0 {
            index_of_first_empty_position_in_to_post = posts[to_post].len() - 1 - index;
            empty_position_found = true;
            break;
        }
    }
    // Sanity check.
    if !empty_position_found {
        panic!("Trying to move to a full post");
    }

    // Move disk.
    posts[to_post][index_of_first_empty_position_in_to_post] = posts[from_post][index_of_topmost_disk_in_from_post];
    posts[from_post][index_of_topmost_disk_in_from_post] = 0;

    // Draw the board so that we can track the progress.
    draw_posts(posts);
}

