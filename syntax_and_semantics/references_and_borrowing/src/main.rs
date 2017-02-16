#![allow(dead_code)]

fn main() {

    // An immutable reference is borrowed
    fn sum_vec(v: &Vec<i32>) -> i32 {
        v.iter().fold(0, |a, &b| a + b)
    }

    // Borrow two vectors and sum them
    // This kind of borrowing does not allow
    // mutation through the borrowed reference
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // Do stuff with v1 and v2
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);

        // Return answer
        s1 + s2
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(&v1, &v2);
    println!("{}", answer);

    // Throws Error!
    // References are immutable, like bindings
    // Cannot borrow immutable borrowed content as immutable
    /*
    fn push_fn(v: &Vec<i32>) {
        v.push(5);
    }

    let v = vec![];

    push_fn(&v);
    */

    //////////////////////
    // &mut references
    //////////////////////

    // Throws error!
    // Cannot borrow 'a' as immutable because it is also borrowed as mutable
    /*
    let mut a = 5;
    
    let b = &mut a;     // mutable borrow occurs here
    *b += 1;
    
    println!("{}", a);  // immutable borrow occurs here
    */

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    /////////////////////////
    // Iterator invalidation
    /////////////////////////

    // Throws error!
    // Cannot mutate a collection that you're iterating over
    // We can't modify 'm' because it's borrowed by the loop
    /*
    let mut m = vec![1, 2, 3];

    for i in &m {           // the immutable borrow prevents subsequent moves 
        println!("{}", i);  // or mutable borrows of 'm' until the borrow ends
        m.push(34);
    }
    */

    let mut m = vec![1, 2, 3];

    for i in &m {
        print!("{} ", i);
    }

    println!();

    ///////////////////
    // Use after free
    ///////////////////

    // Throws error
    // z is only valid for the scope that q exists
    /*
    let z: &i32;
    {
        let q = 5;
        z = &q;     // 'q' does not live long enough
    }
    println!("{}", z);
    */

    // Throws error!
    // Reference is declared BEFORE the variable it refers to
    // 'p' will live longer than 'o'
    // because resources within the same scope are freed
    // in the OPPOSITE order they were declared
    /*
    let p: &i32;
    let o = 5;
    p = &o;         // 'o' does not live long enough

    println!("{}", p);
    */

    // 'o' now lives longer than 'p'
    let o = 5;
    let p: &i32;
    p = &o;

    println!("{}", p);
    
}
