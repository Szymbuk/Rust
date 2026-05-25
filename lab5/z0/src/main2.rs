
#[derive(Debug)]
struct Introduction<'a> {
    intro : &'a str
}

impl Introduction<'_> {
    fn print(&self) {
        println!("{}", self.intro);
    }

    fn get_intro(&self) -> &str{
        self.intro
    }
}


pub fn main2(){
    let t1 = [3,4,5,6,12,14,155];
    let t2 = [9,6,43,2,1,2,46,0,23];
    let res = len_longer_array(&t1,&t2);
    println!("{res}");
    let res = get_longer_array(&t1,&t2);
    println!("{:?}",res);


    //part 2
    let text = String::from("Introduction to a long text. The rest of long text with many sentences.");
    let intro = text.split('.').next().expect("Could not find a first sentence.");
    let i = Introduction { intro };
    i.print();
    let mut res3 = i.get_intro();
    let new_s = res3.replace(" "," - ");
    res3 = &new_s;
    println!("{}",res3);
    i.print();
    println!("{}",i.get_intro());
}

fn len_longer_array(a : &[i32], b : &[i32]) -> usize {
    if a.len() > b.len() {
        a.len()
    } else {
        b.len()
    }
}

fn get_longer_array<'a>(a : &'a[i32], b : &'a[i32]) -> &'a[i32] {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}