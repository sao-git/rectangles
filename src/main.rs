extern crate num;

//use num::PrimInt;
use num::{PrimInt, Integer};

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

struct Screen {
    diagonal: f64,
    height: u32,
    width: u32,
}

impl Screen {
    fn area(&self) -> f64
    where u32: Integer {
        let gcd = self.height.gcd(&self.width);
        let width_ratio = self.width / gcd;
        let height_ratio = self.height / gcd;

        let product = (height_ratio * width_ratio) as f64;
        let sum_2 = sum_of_squares(&[
            height_ratio,
            width_ratio
        ]) as f64;
        let x = self.diagonal / sum_2.sqrt();

        product * x.powf(2.0)
    }
}

fn main() {
    let rect1 = Rectangle {
        width:30, height:50
    };
    let rect2 = Rectangle {
        width:10, height:40
    };
    let rect3 = Rectangle {
        width:60, height:45
    };

    println!(
        "The area of rect1 is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(40);

    println!("square1 = {:?}", square1);

    let screen1 = Screen {
        width:800, height:480, diagonal:7.0
    };

    macro_rules! screen_print {
        () => ("The area of the screen is {:.4} square inches.")
    };

    println!(screen_print!(), screen1.area());

    let screen2 = Screen {
        width:320, height:240, diagonal:3.2
    };

    println!(screen_print!(), screen2.area());

    //println!("{:#?}", rect1);
}



fn sum_of_squares<T>(a: &[T]) -> T
    where T: PrimInt + std::iter::Sum<T> {

    a.iter().map(|x: &T| x.pow(2)).sum()
}

//fn gcd(x: u32, y: u32) -> u32 {
//    let mut x = x;
//    let mut y = y;
//    while y != 0 {
//        let t = y;
//        y = x % y;
//        x = t;
//    }
//    x
//}
