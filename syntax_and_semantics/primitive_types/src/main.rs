use std::mem::size_of_val;

fn main() {
    
    // Booleans
    println!("Booleans");
    let positive_bool = true;
    let negative_bool: bool = false;

    println!("The positive boolean is {}", positive_bool);
    println!("The negative boolean is {}", negative_bool);

    print!("\n");
    println!();

    // Chars
    println!("Chars");

    let my_char = 'x';
    let another_char: char = 'y';

    println!("{} is a character and {} is another character", my_char, another_char);

    println!();
    println!();

    // Numeric types
    println!("Numeric types");

    let default_integer = 45;
    let default_integer_ref = &default_integer; 
    let default_float = 1.0;
    let default_float_ref = &default_float;
    let integer_8: i8 = 10;
    let integer_8_ref = &integer_8;
    let integer_16: i16 = 20;
    let integer_16_ref = &integer_16;
    let integer_32: i32 = 30;
    let integer_32_ref = &integer_32;
    let integer_64: i64 = 40;
    let integer_64_ref = &integer_64;
    let unsigned_8: u8 = 5;
    let unsigned_8_ref = &unsigned_8;
    let unsigned_16: u16 = 15;
    let unsigned_16_ref = &unsigned_16;
    let unsigned_32: u32 = 25;
    let unsigned_32_ref = &unsigned_32;
    let unsigned_64: u64 = 35;
    let unsigned_64_ref = &unsigned_64;
    let variable_integer: isize = 50;
    let variable_integer_ref = &variable_integer;
    let variable_unsigned: usize = 100;
    let variable_unsigned_ref = &variable_unsigned;
    let float_32: f32 = 2.0;
    let float_32_ref = &float_32;
    let float_64: f64 = 5.0;
    let float_64_ref = &float_64;

    println!("The size of a default_integer is {}", size_of_val(default_integer_ref));
    println!("The size of a default_float is {}", size_of_val(default_float_ref));
    println!("The size of a i8 is {}", size_of_val(integer_8_ref));
    println!("The size of a i16 is {}", size_of_val(integer_16_ref));
    println!("The size of a i32 is {}", size_of_val(integer_32_ref));
    println!("The size of a i64 is {}", size_of_val(integer_64_ref));
    println!("The size of a u8 is {}", size_of_val(unsigned_8_ref));
    println!("The size of a u16 is {}", size_of_val(unsigned_16_ref));
    println!("The size of a u32 is {}", size_of_val(unsigned_32_ref));
    println!("The size of a u64 is {}", size_of_val(unsigned_64_ref));
    println!("The size of a isize is {}", size_of_val(variable_integer_ref));
    println!("The size of a usize is {}", size_of_val(variable_unsigned_ref));
    println!("The size of a f32 is {}", size_of_val(float_32_ref));
    println!("The size of a f64 is {}", size_of_val(float_64_ref));

    println!();
    println!();

    // Arrays
    println!("Arrays");

    let my_array = [1, 2, 3];           // my_array: [i32; 3]
    let mut mutable_array = [2, 3, 4];  // mutable_array: [i32; 3]
    
    println!("my_array: {:?}", my_array);
    println!("mutable_array: {:?}", mutable_array);

    mutable_array[1] = 8;

    println!("mutable_array: {:?}", mutable_array);

    let initialised_array = [0; 5];     // initialised_array [i32; 5]

    println!("initialised_array: {:?}", initialised_array);

    let indexes = ["First index", "Second index", "Third index"];

    println!("The first index in the array of indexes is: {:?}", indexes[0]);

    println!();
    println!();

    // Slices
    println!("Slices");

    let number_array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let complete = &number_array[..];
    let middle = &number_array[1..10];

    println!("number_array: {:?}", number_array);
    println!("complete: {:?}", complete);
    println!("middle: {:?}", middle);

    println!();
    println!();

    // Tuples
    println!("Tuples");

    let a_tuple = (1, "Hello");
    let mut another_tuple: (i32, &str) = (45, "Gary");

    println!("a_tuple: {:?}", a_tuple);
    println!("another_tuple: {:?}", another_tuple);

    let assigned_tuple = another_tuple;

    println!("assigned_tuple: {:?}", assigned_tuple);

    let (element_1, element_2, element_3) = (1, 2, 3);

    println!("element_1: {:?}", element_1);

    let single_element_tuple = (5,);

    println!("single_element_tuple: {:?}", single_element_tuple);

    println!("The element at index 1 in assigned_tuple: {:?}", assigned_tuple.1);

    println!();
    println!();

    // Functions
    println!("Functions");

    fn foo(x: i32) -> i32 { x }

    let function_pointer: fn(i32) -> i32 = foo;

    println!("Printing the function foo: {}", foo(5));
    println!("Printing the function pointer: {}", function_pointer(9));


}
