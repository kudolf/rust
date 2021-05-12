
fn main() {
    hello_world();
    variable();
    tuple();
}

// snake case
fn hello_world() {
    println!("Hello, world!");
}

fn variable() {
    let mut x = 5;
    let s = "abc";
    x = x * x;
    let _y = 10;
    let _y = _y + 10;
    println!("{1}, {0}", s, x);
}

fn tuple() {
    let tpl = ((1, 1.0), 1.0);
    println!("{:?}", tpl);
}

