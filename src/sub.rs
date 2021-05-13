struct Point
{
    x: f64,
    y: f64,
}

pub fn sub()
{
    println!("sub");
    struct_func();
}

fn struct_func()
{
    let mut p : Point;
    p = Point { x:1.0, y:2.0 };
    p.x += 10.0;
    println!("{:?}", p.x);
}
