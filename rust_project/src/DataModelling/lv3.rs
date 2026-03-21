// Concept Explanation:
// Rust doesn't have "classes" or inheritance. Instead, you model your domain using structs (grouping data together) and enums (defining a type that can be one of several variants). Paired with Rust's incredibly powerful match statement, this allows you to write highly expressive and exhaustive logic.

// 1.The Rectangle: Define a Rectangle struct with width and height. Write an impl block with a method to calculate its area(), and an associated function square(size) that creates a perfect square.

// 2. IP Addresses: Define an IpAddr enum with two variants: V4 (holding four u8 values) and V6 (holding a String).
#[derive(Debug)]
enum IpAddr{
    V4(u8,u8,u8,u8),  
    V6(String),
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
}

