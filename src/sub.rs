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

pub fn struct_func()
{
    let mut p : Point;
    p = Point { x:1.0, y:2.0 };
    p.x += 10 as f64;
    println!("{:?}, {1}", p.x, p.y);
}

pub fn slice_func()
{
    let xs: [i32; 5] = [1,2,3,4,5];
    analyze_slice(&xs);
    let ys: [i32; 500] = [0; 500];
    analyze_slice(&ys);
}

fn analyze_slice(slice: &[i32])
{
    println!("{}", slice[0]);
    println!("{}", slice.len());
}