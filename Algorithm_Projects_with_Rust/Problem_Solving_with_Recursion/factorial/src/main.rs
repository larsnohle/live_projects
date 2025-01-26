fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
    }
}



fn factorial(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    return n * factorial(n - 1);
}