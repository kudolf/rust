
fn main() {
    hello_world();
    variable();
}

// snake case
fn hello_world() {
    println!("Hello, world!");
}

fn variable() {
    let x = 5;
    let s = "abc";
    println!("{1}, {0}", s, x);
}

