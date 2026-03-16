fn main() {
    println!("Hello World");
    let fibi = fibo(10);
    println!("The 10th fibonacci number is: {}", fibi);
    let t = 100.0;
    println!("The temperature in celsius is: {}", temp(t));
}

fn temp(t:f64) -> f64{
    (t-32.0)*5.0/9.0
}
// fibonaci generator  -- iteration 1
fn fibonacci(n:i32) -> i32 {
    if n <= 1{
        n
    }
    else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

// fibonaci generator  -- iteration 2
fn fibo(n:usize) -> usize{ //what is usize
    let mut a =0;
    let mut b=1;

    for _ in 0..n{
        let temp = a;
        a= b;
        b = temp + a;
        println!("{}",a);
    }
    a

}