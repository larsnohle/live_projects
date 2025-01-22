use std::time::{SystemTime, UNIX_EPOCH};
use std::{fmt, io};
use std::io::Write;
use std::time::Instant;

fn main() {
    let num_items = get_u32("Please enter the number of items: ");
    let max = get_u32("Please enter the max value of an item: ");
    let vector_to_sort = make_random_vec(num_items, max);
    let now = Instant::now();
    let sorted_vector = counting_sort(&vector_to_sort, max);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    check_sorted(&sorted_vector);
}

fn counting_sort(vector_to_sort: &[Customer], max_item_value: u32) -> Vec<Customer> {
    // Create an output vector of the same length as the input vector.
    let mut output_vector: Vec<Customer> = Vec::with_capacity(vector_to_sort.len());

    // Create a counts vector, i.e. the vector called C in the brilliant.org article.
    let mut counts = vec![0u32; max_item_value as usize];

    // Loop through the input vector and count the frequency of each value.
    // At the same time, create an "empty" Customer and push it to the output vector.
    // It is these Customer structs we later set correct values for when we use the counts vector
    // to create the correct sort order.
    vector_to_sort.iter().for_each(|v| {
        counts[v.num_purchases as usize] += 1;
        output_vector.push(Customer {
            id: "".to_string(),
            num_purchases: 0
        })
    });

    // Update the counts vector so that each element contains the number of elements in the
    // input vector that is less than or equal to the index of the element in the counts vector.
    for i in 1..counts.len() {
        counts[i] += counts[i - 1]
    }

    // Insert values into the output vector.
    // Loop through the input vector in reverse.
    for customer in vector_to_sort.iter().rev() {
        let index_in_counts_vector = customer.num_purchases as usize;
        let index_to_insert_value_at: usize = counts[index_in_counts_vector] as usize - 1; // -1 as indexing is, well, zero based.
        // Copy the values from the element in the input vector to the element in the output vector.
        output_vector[index_to_insert_value_at].id = (*customer.id.clone()).to_string();
        output_vector[index_to_insert_value_at].num_purchases = customer.num_purchases;

        counts[index_in_counts_vector] -= 1;
    }
    output_vector
}


// Prints a message indicating if the specified vector is sorted or not.
fn check_sorted(vec: &[Customer]) {
    let mut is_sorted = true;
    for i in 0..vec.len() - 1 {
        if vec[i].num_purchases > vec[i + 1].num_purchases {
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
fn get_u32(prompt: &str) -> u32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<u32>()
        .expect("Error parsing integer")
}

// Print at most num_items items.
fn print_vec(vec: &[Customer], num_items: u32) {
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
fn make_random_vec(num_items: u32, max: u32) -> Vec<Customer> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<Customer> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        let mut customer_id = "C".to_string();
        customer_id.push_str(i.to_string().as_str());
        let customer = Customer {
            id: customer_id,
            num_purchases: prng.next_u32_below_max(max),
        };
        vec.push(customer);
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

    // Return a pseudorandom value in the range [0, max).
    pub fn next_u32_below_max(&mut self, max: u32) -> u32 {
        let result = max as f64 * self.next_f64();
        result as u32
    }
}

#[derive(Debug)]
struct Customer {
    id: String,
    num_purchases: u32,
}
impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

