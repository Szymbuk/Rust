struct Rectangle {
    width : f64,
    height : f64
}

impl Rectangle {
    fn new(width : f64, height : f64) -> Result<Rectangle,String> {
        if width<=0.0 || height <=0.0{
            let message = String::from("Długość boku nie może być ujemna"))
            return Err(message)
        }
        Ok(Rectangle{width, height})
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn main3(){

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rectangle() {
        // given
        let width = 4.5;
        let height = 5.7;

        // when
        let r = Rectangle::new(width, height).unwrap();

        // then
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic]
    fn test_negative_rectangle_sides(){
        let r = Rectangle::new(-1.0, 1.0);
        match r {
            Err(s) => assert_eq!("Rectangle cannot have negative width or height", s.as_str()),
            Ok(_) => panic!() // the result shouldn't be Ok
        } 

    }
}