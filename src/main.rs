mod sub;

fn main() {
    hello_world();
    variable();
    tuple();
    array();
    boolean();
    sub::sub();
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

fn array() {
    let ary = [1, 2];
    println!("{0}", ary[0]);
    println!("{:?}", ary);

    let ary2:[u32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", ary2);
}

fn boolean() {
    let b = true;
    println!("true:{0}, false:{1}, [variable]b:{2}", true, false, b);
}
