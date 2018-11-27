extern crate num_rational;
extern crate num_traits;
mod shapes;
mod math;

use shapes::screen::Screen;

macro_rules! screen_print {
    () => ("The area of the screen is {:.4} square inches.")
}

//macro_rules! debug_print {
//    ($x:ident) => ({("$x = "), $x})
//}

fn main() {
    let screen1 = Screen::new(
        7.0, 800, 480
    );
    let screen1_area = screen1.area();
    println!(screen_print!(), screen1_area);

    let screen2 = Screen::new(
        3.2, 320, 240
    );
    let screen2_area = screen2.area();
    println!(screen_print!(), screen2_area);

    let percent_diff = screen1_area / screen2_area - 1.0;
    println!(
        "Percent difference in areas: {:.4} %",
        percent_diff * 100.0
    );

    let numbers = vec![2.3, 4.5, -23333.0123];
    //let numbers = vec![2, 4, -23333];
    let numbers_2 = math::sum_pow(&numbers, 5);

    println!("numbers_2 = {}", numbers_2);

    let numbers_3 = math::sum_pow(&[
            5.0_f64.sqrt(),
            5.0_f64.sqrt()], 2);
    println!("numbers_3 = {}", numbers_3);

    let numbers_4 = math::mean(&numbers, 5);
    println!("numbers_4 = {}", numbers_4);

    let numbers_5 = math::rms(&numbers);
    println!("numbers_5 = {}", numbers_5);
    //println!(debug_print!(numbers_5));
}
