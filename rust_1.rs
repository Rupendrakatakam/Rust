fn main() {
    let fib = fibonacci(10);
    println!("The 10th fibonacci number is: {}", fib);
}

// fibonaci generator 
fn fibonacci(n:i32) -> i32 {
    if n <= 1{
        n
    }
    else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}
    