fn main() {
    // bindings are immutable
    let x = 5;
    // x = 10;  // this will fail

    // make a binding mutable
    let mut x = 10;
    x = 5;

    // variables must be initialised before use
    let y: i32;

    // this will not work
    // println!("The value of y is: {}", y);

    println!("\n");
    // It will work when y is bound to a value
    y = 15;
    println!("The value of y is: {}", y);

    {
        let z = 60;
        println!("The value of x is: {}.  The value of z is: {}", x, z);
    }

    // z is out of scope, so this will not work
    // println!("The value of x is: {}.  The value of z is: {}", x, z);

    println!("\n");
    // shadowing and scope
    {
        println!("x is {}", x);
        let x = 90;
        println!("x is {}", x);
    }
    println!("x is {}", x);
    let x = 30;
    println!("x is {}", x);

    println!("\n");
    let mut x: i32 = 50;
    println!("x is {}", x);
    x = 7;
    println!("x is {}", x);
    let x = x;
    println!("x is {} and is also immutable.", x);

    println!("\n");
    let y = 20;
    println!("y is {}", y);
    let y = "now a string!";
    println!("y is {}", y);
}
