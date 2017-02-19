#![allow(dead_code)]

// In structs
struct Foo<'a> {
    x: &'a i32,
}

// impl blocks
impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {self.x}
}

fn main() {

    /*
    // This code will not work!
    // Reference points to variable that has gone out of scope
    // This creates what's called a "dangling pointer"

    let r;                  // introduce reference: 'r'
    {
        let i = 1;          // introduce scoped value: 'i'
        r = &i;             // store reference of 'i' in 'r'
    }
    println!("{}", r);      // 'r' still refers to 'i'
    */

    /*
    // Lifetimes will need to be make explicit when functions are involved
    // The below code will not compile

    fn skip_prefix(line: &str, prefix: &str) -> &str {
        // ...
    }

    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);  // -+ 'p' comes into scope
        v = skip_prefix(line, p.as_str());  //  |
    }                                       // -+ 'p' goes out of scope
    println!("{}", v);
    */

    /*
    // To get it to compile the above function will need to be declared like:

    fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
        // ..
    }
    */

    // In structs
    let y = &5;         // this is the same as 'let _y = 5; let y = _y;'
    let f = Foo { x: y };

    // impl blocks

    // before impl
    println!("{}", f.x);

    // after impl
    println!("x is: {}", f.x());

    // multiple lifetimes
    // fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {

    //Thinking in scopes
    let q = &5;             // -+ 'y' comes into scope
    let f = Foo { x: y };   // -+ 'f' comes into scope
                            //  |
                            // 'f' and 'y' go out of scope at the end of this block
    /*
    // Doing this would not work
    let x;                      // -+ 'x' comes in scope
    {                           //  |
        let p = &5;             // -+ 'p' comes in scope
        let o = Foo { x: y };   // -+ 'o' comes in scope
        x = &o.x;               //  |   this causes an error when 'o' goe out of scope
    }                           // -+ 'p' and 'o' go out of scope
    println!("{}", x);          // 'x' goes out of scope at the end of this block
    */

    // 'static

    // string literals
    let u: &'static str = "Hello, world.";

    // globals
    static GLOB: i32 = 5;
    let t: &'static i32 = &GLOB;

    // Lifetime elision
    /*
    // input lifetime
    // fn foo<'a>(bar: &'a str)

    // output lifetime
    // fn foo<'a>() -> &'a str

    // both input and output lifetime
    fn foo<'a>(bar: &'a str) -> &'a str
    */

    // Examples
    /*
    fn print(s: &str);                                                          // elided
    fn print<'a>(s: &'a str);                                                   // expanded

    fn debug(lvl: u32, s: &str);                                                // elided
    fn debug<'a>(lvl: u32, s: &'a str);                                         // expanded

    fn substr(s: &str, until: u32) -> &str;                                     // elided
    fn substr<'a>(s: &'a str, until: u32) -> &'a str;                           // expanded

    fn get_str() -> &str;                                                       // ILLEGAL, no inputs

    fn frob(s: &str, t: &str) -> &str;                                          // ILLEGAL, two inputs
    fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str;                            // Expanded: Output lifetime is ambiguous

    fn get_mut(&mut self) -> &mut T;                                            // elided
    fn get_mut<'a>(&'a mut self) -> &'a mut T;                                  // expanded

    fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command;                  // elided
    fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; //expanded

    fn new(buf: &mut [u8]) -> BufWriter;                                        // elided
    fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>;                             // expanded
    */

}
