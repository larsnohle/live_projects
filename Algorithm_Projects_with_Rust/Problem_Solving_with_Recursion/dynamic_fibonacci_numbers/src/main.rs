use std::io;
use std::io::Write;

fn main() {
    // Create a vector for fill-on-the-fly.
    let mut fill_on_the_fly_values: Vec<i64> = Vec::new();
    fill_on_the_fly_values.push(0);
    fill_on_the_fly_values.push(1);

    // Initialize the prefilled vector.
    let prefilled_values = prefill_vector();

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // Calculate the Fibonacci number.
        println!("Prefilled:  {}", prefilled_values[n as usize]);
        println!("On the fly: {}", fibonacci_on_the_fly(&mut fill_on_the_fly_values, n));
        println!("Bottom up:  {}", fibonacci_bottom_up(n));
        println!();
    }

}

fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    let index_of_n = n as usize;
    if values.len() > index_of_n {
        return values[index_of_n];
    }

    let fibonacci_n = fibonacci_on_the_fly(values, n - 2) + fibonacci_on_the_fly(values, n - 1);
    values.push(fibonacci_n);
    fibonacci_n
}

fn prefill_vector() -> Vec<i64> {
    let mut values: Vec<i64> = Vec::new();
    let mut previous_previous_value = 0;
    let mut previous_value = 1;
    values.push(previous_previous_value);
    values.push(previous_value);


    for _ in 2..=92 {
        let current_value = previous_value + previous_previous_value;
        values.push(current_value);
        previous_previous_value = previous_value;
        previous_value = current_value;
    }
    values
}

fn fibonacci_bottom_up(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut fib_i_minus_2 = 0i64;
    let mut fib_i_minus_1 = 1i64;
    let mut fib_i = fib_i_minus_1 + fib_i_minus_2;
    for _ in 1i64..n {
        // Calculate this value of fib_i.
        fib_i = fib_i_minus_1 + fib_i_minus_2;

        // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
        fib_i_minus_2 = fib_i_minus_1;
        fib_i_minus_1 = fib_i;
    }
    fib_i
}

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i64>()
        .expect("Error parsing integer")
}