use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::io;
use std::io::Write;

fn main() {
    // let mut v = &mut[9,3,1,4,1,5,9,2,6,5,3,5];
    // println!("{:?}",v);
    // quicksort(v);

    let num_items = get_i32("Please enter the number of items: ");
    let max = get_i32("Please enter the max value of an item: ");
    let mut vector_to_sort = make_random_vec(num_items, max);
    //print_vec(&vector_to_sort, max);
    let now = Instant::now();
    quicksort(&mut vector_to_sort);
    //print_vec(&vector_to_sort, max);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    check_sorted(&vector_to_sort);
}

fn quicksort(slice_to_sort: &mut [i32]) {
    // An empty array or a one-element array is already sorted.
    let number_of_elements = slice_to_sort.len();
    if  number_of_elements <= 1 {
        return;
    }

    // Partition the slice...
    let pivot_index = partition(slice_to_sort);

    // And sort the resulting sub-slices.
    quicksort(&mut slice_to_sort[0..pivot_index]);
    quicksort(&mut slice_to_sort[pivot_index..number_of_elements]);
}

/// Partitions the specified slice into two part: one part containing elements that
/// are less or equal to the pivot element (which is chosen to the last element) and one part
/// containing elements that are greater or equal to the pivot element.
///
/// Returns the index of the last element in the first partition.
///
/// # Arguments
///
/// * `sliceToPartition` - The slice that should be partitioned.
fn partition(slice_to_partition: &mut [i32]) -> usize {
    // Pick the last element as the pivot element.
    let pivot = slice_to_partition[slice_to_partition.len() - 1];

    // After looping has started, elements to the left of this index is <= the pivot index.
    let mut working_index: usize = 0;

    // Loop through all but the last elements of the slice.
    for j in 0..slice_to_partition.len() - 1 {
        // If the element in question is less or equal to the pivot element, we swap it and the one pointed to by the working index.
        if slice_to_partition[j] <= pivot {
            // However, there is no need to swap if the indices are the same.
            if j != working_index {
                slice_to_partition.swap(working_index, j);
            }
            // We need to advance the working_index pointer though.
            // Advance the pointer pointing to the element we're comparing with.
            working_index += 1;
        }
    }

    // Now all elements to the left of the working index is <= the pivot element.
    // The element pointed to by the working index is bigger or equal to the pivot element (if the slice is sorted).
    // When we reach this point, all elements to the right of working_index is greater or equal to the pivot element.
    // As we want all elements having index <= the returned index to be LESS OR EQUAL to the pivot element,
    // we now need to swap the element at the working index (because it might point to an element that is GREATER
    // than the pivot element) and the pivot element. By doing this, we ensure that all elements having index
    // <= the returned index are <= the pivot element.
    slice_to_partition.swap(working_index, slice_to_partition.len() - 1);

    // Return the index of the pivot element.
    working_index
}


// Prints a message indicating if the specified vector is sorted or not.
fn check_sorted(vec: &[i32]) {
    let mut is_sorted = true;
    for i in 0..vec.len() - 1 {
        if vec[i] > vec[i + 1] {
            is_sorted = false;
            break;
        }
    }

    if is_sorted {
        println!("The vector is sorted!");
    } else {
        println!("The vector is NOT sorted!");
    }
}

// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i32>()
        .expect("Error parsing integer")
}

// Print at most num_items items.
fn print_vec(vec: &[i32], num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push('[');

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push(' ');
        string.push_str(&vec[i].to_string());
    }
    string.push(']');
    println!("{string}");
}

// Make a vector of random i32 values in the range [0 and max).
fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    vec
}

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        prng
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        self.seed
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    pub fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        f / (2147483647.0 + 1.0)
    }

    // Return a pseudorandom value in the range [min, max).
    pub fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        result as i32
    }
}