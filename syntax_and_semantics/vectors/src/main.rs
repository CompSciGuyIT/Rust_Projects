#![allow(dead_code)]

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    let w = vec![0; 10];

    // Accessing elements

    let u: usize = 0;
    let i: i32 = 0;

    println!("The first element in the vector is {}", v[u]);
    println!("Cannot index vectors with non-usize values");

    // Out-of-bounds access

    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None    => println!("Sorry, the vector is out of bounds!"),
    }

    // Iterating

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {                               // Vector must have been declared mutable first
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its elements {}", i);
    }

    println!("Once ownership is taken, the code will not compile if attempting to use the vector again!");

}
