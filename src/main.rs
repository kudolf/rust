
fn main() {
    hello_world();
    variable();
}

// snake case
fn hello_world() {
    println!("Hello, world!");
}

fn variable() {
    let mut x = 5;
    let s = "abc";
    x = x * x;
    println!("{1}, {0}", s, x);
}

