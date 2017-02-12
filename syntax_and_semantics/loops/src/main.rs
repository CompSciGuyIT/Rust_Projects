fn main() {

    /*************************************
    
    loop {
        println!("This loops forever!");
    }
    
    *************************************/

    // While loop

    let mut x = 5;
    let mut done = false;

    println!("While loop");
    
    while !done{
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }

    // For loop

    println!();
    
    println!("For loop");
    
    for x in 0..10 {
        println!("{}", x);
    }

    // For with enumeration

    println!();
    
    println!("For with enumeration");
    
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    x = 5;

    // break and continue

    println!();
    
    println!("break and continue");
    
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break;}
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }

    // Labelling loops

    println!();

    println!("Labelling loops");
    
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
}
