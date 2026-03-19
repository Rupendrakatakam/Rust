fn main() {
    println!("Hello, world!");

    let x= String::from("hello5441 world a3131 awd542");
    let _y= x.clone();
    
    println!("{}", x);
    let z = string_length(&x);
    println!("{}", z);
    let a = string_slice(&x);
    println!("{}",a);
    let b = reverse_sentence(&x);
    println!("{}",b);
}

fn string_length(s: &String) -> usize {
    s.len()
}

fn string_slice(s: &str)-> &str{
    &s[0..=3]
}

// Mini Project: Sentence Reverser
// Write a program that takes a sentence String, splits it into words, and reconstructs the sentence with the words in reverse order. The catch: You must do this using string slices (&str) to avoid allocating new memory for the individual words on the heap.

fn reverse_sentence(sentence: &str) -> String {
    sentence.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}