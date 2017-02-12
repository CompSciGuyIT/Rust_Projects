fn main() {
    let x = 5;

    if x == 5 {
        println!("x is equal to 5");
    } else if x == 6 {
        println!("x is equal to 6");
    } else {
        println!("x is neither 5 or 6");
    }

    let y = if x == 5 {10} else {15}; // y: i32
    println!("y is equal to {}", y);
}
