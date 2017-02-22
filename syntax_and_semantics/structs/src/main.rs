#![allow(dead_code)]
#![feature(field_init_shorthand)]

struct Point {
    x: i32,
    y: i32,
    // mut z: i32,  // Rust does not support field mutability
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,    
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

struct Color(i32, i32, i32);
struct Point_T(i32, i32, i32);

struct Inches(i32);

struct Electron {}
struct Proton;

fn main() {
    let origin_x = 0;
    let origin_y = 0;

    let origin = Point { x: 0, y: 0 };  // origin: Point

    println!("The origin is at {}, {}", origin.x, origin.y);

    let mut point = Point { x: 0, y: 0 };
    point.x = 5;

    println!("The point is at ({}, {})", point.x, point.y);

    let point = point;  // point is now immutable

    // point.y = 6;     // This will now cause an error

    println!();

    let mut point2 = Point { x: 0, y: 0 };

    println!("The point2 is at ({}, {})", point2.x, point2.y);

    {
        let r = PointRef { x: &mut point2.x, y: &mut point2.y };
        
        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point2.x);
    assert_eq!(6, point2.y);

    println!("The point2 is at ({}, {})", point2.x, point2.y);

    println!();

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    //////////////////////////
    // Update syntax

    println!();

    let mut point3 = Point3d { x: 0, y: 0, z: 0 };
    point3 = Point3d { y: 1, .. point3 };       // gives point3 a new 'y', but keeps the old 'x' and 'z' values        

    println!("point3: ({}, {}, {})", point3.x, point3.y, point3.z);

    ///////////////////////////
    // Tuple structs

    println!();

    let black = Color(0, 0, 0);
    let origin = Point_T(0, 0, 0);

    let black_r = black.0;
    let Point_T(_, origin_y, origin_z) = origin;

    let length = Inches(10);

    let Inches(integer_length) = length;
    // let integer_length = length.0;       // This would do the same thing

    println!("length is {} inches", integer_length);

    //////////////////////////
    // Unit-like structs

    let x = Electron {};
    let y = Proton;
}
