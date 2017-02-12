//! Doc comment to comment containing items
//! (e.g. crates, modules, functions)

// Comments

// Single line comments

/*
    Block comments
*/

/// Doc comments
/// supports markdown notation
/// examples can be used as tests
///
/// let five = 5;
///
/// assert_eq!(6, add_one(five));
/// # fn add_one(x: i32) -> i32 {
/// #   x + 1
/// # }
/// 

fn add_one(x: i32) -> i32 {
    x + 1
} 


fn main() {
    let five = 5;
    println!("{} is {} minus one", five, add_one(five));
}
