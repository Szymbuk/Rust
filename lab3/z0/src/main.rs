use std::fmt::{Display, Formatter};

trait Shape{

    fn describe(&self) {
        println!("I'm a general shape.");
    }

    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

trait Area{
    fn area(&self) -> f32;
}


#[derive(Debug)]
struct Rectangle{x:f32, y:f32}


impl Shape for Rectangle{
    fn describe(&self) {
        println!("I'm a rectangle.");
    }

    fn area(&self)-> f32{
        self.x*self.y
    }

    fn perimeter(&self) -> f32{
        2f32*self.x + 2f32*self.y
    }
}

impl Area for Rectangle{
    fn area(&self)-> f32{
        self.x*self.y
    }
}

impl Rectangle {
    fn area(&self)-> f32{
        self.x*self.y
    }

    fn perimeter(&self) -> f32{
        0.01f32*self.x + 0.01f32*self.y
    }


    fn scale(&mut self, factor: f32){
        self.x *= factor;
        self.y *= factor;
    }

    fn new_square(x: f32) -> Rectangle{
        Rectangle{x, y: x}
    }
}

impl Display for Rectangle{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle[x={},y={}]", self.x, self.y)?;
        Ok(())
    }
}


fn main() {

    let r = Rectangle{x:3f32,y:4f32};

    let r2 =Rectangle{x:3f32,..r};
    println!("{:?}",r);
    println!("x: {}, y: {}", r2.x, r2.y);


    let r3 = Rectangle{x: 5f32, y:9f32};
    println!("[{}, {}]",r3.x,r3.y);
    //r3.x =10f32;


    let mut r4 = Rectangle{x: 5f32, y:9f32};
    println!("[{}, {}]",r4.x,r4.y);
    r4.x =10f32;
    r4 .y = 7f32;
    println!("[{}, {}]",r4.x,r4.y);


    println!("Area of {:?} is {}", r4, r4.area());
    println!("Paremeter of {:?} is {}", r4, r4.perimeter());
    r4.scale(3f32);
    println!("Area of {} is {}", r4, r4.area());
    println!("Paremeter of {} is {}", r4, r4.perimeter());

    println!("\n\n");

    let square = Rectangle::new_square(4f32);
    println!("{:?}",square);
    square.describe();
}
