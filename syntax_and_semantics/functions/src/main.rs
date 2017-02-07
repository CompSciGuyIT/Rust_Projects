fn main() {
    print_number(5);
    print_sum(4, 5);

    println!("9 becomes {}", add_one(9));
    println!("9 is entered, {} is returned", returns_odd(9));
    println!("2 is entered, {} is returned", returns_odd(2));

    let a: fn(i32) -> i32 = add_one;
    let six = a(5);
    println!("The value bound to six is: {}", six);

    diverges();
}

// argument types must be specified in functions
fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("the sum of x and y is {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// return types are specified after the '->'
fn returns_odd(x: i32) -> i32 {
    if x % 2 == 0 {
        println!("The number has been made odd.");
        return x + 1;
    }
    x
}

// panic! causes the current thread to crash
// the return type is '!' because the function never returns
fn diverges() -> ! {
    panic!("Yikes!  This function never returns!");
}


