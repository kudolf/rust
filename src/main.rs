mod sub;

fn main() {
    hello_world();
    variable();
    tuple();
    array();
    boolean();
    sub::sub();
    sub::struct_func();
    sub::slice_func();
    sub::hitpoint_func();
    sub::type_inference();
    ifelse();
    loop_func();
    clone_string();
    clone_int();
    nest_and_label();
    crate::game::network::func1(); // 絶対パス
    crate::game::battle::func1();  // 絶対パス
    game::network::func1(); // 相対パス
    game::battle::func1();  // 相対パス
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

fn ifelse()
{
    let a = 10;
    let b = 
        if a > 10
        {
            1
        } else if a < 10 {
            2
        } else {
            3
        };
    println!("{:?}", b);
}

fn loop_func()
{
    let mut count = 0;
    loop
    {
        count += 1;
        if count > 5
        {
            break;
        }
    }
}

fn clone_string()
{
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{:1}, {:2}", s1, s2);
}

fn clone_int()
{
    let i1 = 100;
    let i2 = i1.clone();
    println!("{:1}, {:2}", i1, i2);
}

fn nest_and_label()
{
    let mut i = 0;
    'outer: loop
    {
        'inner: loop
        {
            i += 1;
            if i > 5
            {
                println!("break {:1}", i);
                break 'outer;
            }
        }
    }
}

/*
crate
└game
 └network
 └battle
 */
mod game
{
    pub mod network
    {
        pub fn func1()
        {
            println!("network::func1");
        }
    }

    pub mod battle
    {
        pub fn func1()
        {
            println!("battle::func1");
        }
    }
}
