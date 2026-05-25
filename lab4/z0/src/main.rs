#[derive(Debug)]
enum UIEvent{
    ButtonClicked,
    Scroll(Direction),
    KeyPressed(char),
    MouseClicked{x:i32, y:i32}
}

impl UIEvent{
    fn describe(&self){
        println!("{:?}",self)
    }
}

#[derive(Debug)]
enum Direction{
    UP,
    DOWN
}

#[derive(Debug)]
enum Message{
    Move{x:i32, y:i32},
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit
}
impl Message{
    fn call(&self){
        println!("{:?}",self);
    }
}



use UIEvent::*;
use Direction::*;

fn main() {
    let clicked = ButtonClicked;
    let scroll = Scroll(UP);
    scroll.describe();
    let key_pressed = KeyPressed('c');
    let mouse_clicked = MouseClicked {x:10,y:10};
    UIEvent::describe(&mouse_clicked);

    call(clicked);
    call(scroll);
    call(key_pressed);
    call(mouse_clicked);

    println!("\n\n");
    //main2();
}


fn main2(){
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

fn call(event: UIEvent){
    match event{
        ButtonClicked => println!("Button clicked"),
        Scroll(x) => println!("Scroll {:?}",x),
        KeyPressed(ch) => {
            let c = ch.to_uppercase();
            println!("Key pressed {c}");
        }
        UIEvent::MouseClicked {x,y} => println!("Mouse clicked ({x}, {y})")
    }
}
