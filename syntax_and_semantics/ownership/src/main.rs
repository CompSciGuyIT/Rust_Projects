#![allow(dead_code)]

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;

    // This will throw an error
    // error: use of moved value: 'v'
    // print!("{}", v[0]);

    fn take(v: Vec<i32>) {
        // nothing happens here
    }

    let vec1 = vec![1, 2, 3];
    take(vec1);

    // This will throw an error
    // error: use of moved value: 'vec1'
    // print!("{}", vec1[0]);

    let i = 10;
    let i2 = i;

    println!("i is an i32 primitive type: {}", i);

    let i3 = double(i);
    
    println!("i: {}, i3: {}", i, i3);

    let b = true;
    let b2 = change_truth(b);

    println!("b: {}, b2: {}", b, b2);

    println!();

    println!("Handing back ownership can become tedius.");

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let (v1, v2, ans) = foo(v1, v2);

    println!("v1: {:?}\nv2: {:?}\nans: {}", v1, v2, ans);
    println!("Borrowing can help solve this problem.")
}

fn double(x: i32) -> i32 {
    x * 2
}

fn change_truth(x: bool) -> bool {
    !x
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // Hand back ownership and the the result of the function
    (v1, v2, 43)
}