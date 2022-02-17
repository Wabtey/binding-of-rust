#![allow(unused)]
fn lesson() {
    // cmd+maj+/ put in comment the slection

    let url = "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html";

    let introduction = 
    "Ownership is Rust’s most unique feature and has deep implications \n
    for the rest of the language. \n
    It enables Rust to make memory safety guarantees
    without needing a garbage collector, \n
    so it’s important to understand how ownership works.";

    println!("{}", introduction);

    let where_you_belong =
    "All data stored on the stack must have a known, fixed size. \n
    Data with an unknown size at compile time or a size that might change
    must be stored on the heap instead.";

    let ownership_rules = "
    Each value in Rust has a variable that’s called its owner. \n
    There can only be one owner at a time. \n
    When the owner goes out of scope, the value will be dropped.";
}

fn main(){
    lesson();

}
