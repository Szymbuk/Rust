struct Point{
    x: i32,
    y: i32
}




fn main() {
    let x =10u8;

    match x{
        1 | 4 => println!("one or four"),
        2 | 5=> println!("two or five"),
        3 | 6=> println!("three or six"),
        _ => println!("other"),
    }

    match x {
        0..=9 => println!("small"),
        10..=99 => println!("medium"),
        100..=255  => println!("big"),
        //_ => println!("something else")
    }


    let p = Point{x:0,y:7};

    match p {
        Point{x, y:0} => println!("On the x axis at {x}"),
        Point{x:0, y} => println!("On the y axis at {y}"),
        Point{x, y} => println!("On neither axis: ({x}, {y})"),
    }
}
