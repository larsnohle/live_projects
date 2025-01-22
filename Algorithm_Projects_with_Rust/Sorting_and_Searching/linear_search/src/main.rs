use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use std::io::Write;

fn main() {
    let num_items = get_i32("Please enter the number of items: ");
    let max = get_i32("Please enter the max value of an item: ");
    let vector_search_in = make_random_vec(num_items, max);
    print_vec(&vector_search_in, 40);

    let mut done = false;
    while !done {
        let value_to_search_for = get_i32("Please enter the value to search for: ");
        if value_to_search_for == -1 {
            done = true;
        }

        let (index_of_number, number_of_tests_performed) = linear_search(&vector_search_in, value_to_search_for);
        println!("Index of number is {index_of_number}");
        println!("Number of tests performed is: {number_of_tests_performed}");
    }

    print_vec(&vector_search_in, 40);
    check_sorted(&vector_search_in);
}


fn linear_search(numbers: &[i32], target: i32) -> (i32, u32) {
    let mut number_of_tests_performed: u32 = 0;
    let mut index_of_number: i32 = -1;
    for i in 0..numbers.len() {
        number_of_tests_performed += 1;
        if numbers[i] == target {
            index_of_number = i as i32;
            break;
        }
    }

    (index_of_number, number_of_tests_performed)
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