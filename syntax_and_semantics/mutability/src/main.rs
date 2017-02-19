#![allow(dead_code)]

use std::sync::Arc;
use std::cell::RefCell;
use std::cell::Cell;

fn main() {
    let mut x = 5;
    let y = &mut x;     // 'y' is an immutable binding to a mutable reference

    let a = Arc::new(5);
    let b = a.clone();

    let p = RefCell::new(42);
    let o = p.borrow_mut();
    // let i = p.borrow_mut();  // will cause panic! at runtime

    struct Point {
        x: i32,
        // mut y: i32;  // can't have mutable and immutable fields
        y: i32,
    }

    let mut m = Point { x: 5, y: 6 };
    m.x = 10;           // mutability is in the binding

    let n = Point { x: 4, y: 9 };
    // n.x = 8;         // cannot assign to immutable field n.x 

    struct Point2 {
        x: i32,
        y: Cell<i32>,
    }

    let point = Point2 { x: 5, y: Cell::new(4) };

    point.y.set(8);

    println!("y: {:?}", point.y);
}
