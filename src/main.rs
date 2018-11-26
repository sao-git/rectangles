extern crate num_rational;
extern crate num_traits;
mod shapes;
mod math;

use shapes::screen::Screen;

fn main() {
    macro_rules! screen_print {
        () => ("The area of the screen is {:.4} square inches.")
    };

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
    let numbers_2 = math::sum_of_squares(&numbers);

    println!("numbers_2 = {}", numbers_2);

    let numbers_3 = math::sum_of_squares(&[
            2.0_f64.sqrt(),
            2.0_f64.sqrt()]);
    println!("numbers_3 = {}", numbers_3);
}
