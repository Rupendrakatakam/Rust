// Concept Explanation:
// Rust doesn't have "classes" or inheritance. Instead, you model your domain using structs (grouping data together) and enums (defining a type that can be one of several variants). Paired with Rust's incredibly powerful match statement, this allows you to write highly expressive and exhaustive logic.

// 1.The Rectangle: Define a Rectangle struct with width and height. Write an impl block with a method to calculate its area(), and an associated function square(size) that creates a perfect square.

// 2. IP Addresses: Define an IpAddr enum with two variants: V4 (holding four u8 values) and V6 (holding a String).

// Coin Sorter: Create a Coin enum (Penny, Nickel, Dime, Quarter). Write a function that uses a match expression to return the value of the coin in cents.

// Mini Project: Shape Area Calculator
// Create an enum called Shape with variants Circle(f64) (holding the radius) and Rectangle(f64, f64) (holding width and height). Write a function calculate_area(shape: &Shape) -> f64 that uses a match statement to compute the area regardless of the shape passed in.

#[derive(Debug)]
enum IpAddr{
    V4(u8,u8,u8,u8),  
    V6(String),
}

enum Coin{
    Penny, Nickel, Dime, Quarter
}

enum Shape{
    circle(f64), rectangle(f64,f64)
}

struct Rectangle{
    length : u32,
    width: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn sq(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            length: size,
        }
    }
}

fn print_ip(ip : &IpAddr){
    match ip {
        IpAddr::V4(a,b,c,d) => println!("{}.{}.{}.{}",a,b,c,d),
        IpAddr::V6(s) => println!("{}",s),
    }
}

fn coin_sort(coin: &Coin) -> u32 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn calculate_area(shape: &Shape) -> f64{
    match shape{
        Shape::circle(r) => 3.14 * r * r,
        Shape::rectangle(w,h) => w * h,
    }
}

fn main(){
    let rect1 = Rectangle{ width : 10, length : 10};
    println!("Area of rectangle is {}", rect1.area());
    let square = Rectangle::sq(4);
    println!("area of square :{}",square.area());

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    // println!("home : {:?} and loopback : {:?}", home, loopback)
    // println!("home : {:?}", home)
    print_ip(&home);
    print_ip(&loopback);

    let coin = Coin::Penny;
    println!("coin value : {}", coin_sort(&coin));

    let x = calculate_area(&Shape::circle(5.0));
    println!("area of circle : {}", x);
}

