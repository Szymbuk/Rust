//mod main2;

mod main2;
mod main3;

use std::ops::Add;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T
}


impl <T> Pair<T> {
    fn extract_x(self) -> T {
        self.x
    }

}

impl <T: PartialOrd+Copy> Pair<T> {
    fn bigger(&self) -> T{
        if self.x> self.y { self.x }
        else { self.y }
    }
}


fn main() {
    // let pi = Pair{x : 5, y : 3};
    //
    // let pf = Pair {x: 15f64, y : 12.0f64};
    //
    //
    //
    // //println!("{:?} {}",pi.bigger(),pi.extract_x());
    // //println!("{:?} {}",pf.bigger(),pf.extract_x());
    //
    // let ar1 = [3,4,2,1,95,12,43];
    // let ar2:[i32;0] = [];
    // println!("{:?}",max(&ar1));
    // println!("{:?}",max(&ar2));
    // println!("{:?}",mean(&ar1));
    //main2::main2()
    main3::main3()
}



fn max<T: Sized + PartialOrd + Copy>(ar: &[T]) -> Option<T>{
    if ar.len()==0{ return None }

    let mut maxi = ar[0];
    for item in ar {
        if *item > maxi {
            maxi = *item;
        }
    }
    Some(maxi)
}



fn mean<T:Copy+Into<f64> > (ar: &[T]) -> Option<f64>{
    let l = ar.len();
    if l==0{ return None }

    let mut sum:f64 = 0f64;
    for item in ar{
        sum+= (*item).into();
    }
    Some(sum/(l as f64))
}