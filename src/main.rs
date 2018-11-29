extern crate num_rational;
extern crate num_traits;
mod shapes;
mod math;

use shapes::rectangle::RealRectangle;
use shapes::screen::Screen;
use shapes::ratio::HasRatio;

macro_rules! screen_print {
    () => ("The area of the screen is {} square inches.")
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
    let screen1_lengths = screen1.side_lengths();
    println!("screen1 lengths in inches: {}, {}",
             screen1_lengths.width,
             screen1_lengths.height
    );
    let area1 = screen1_lengths.area();
    println!("Area using side lengths: {}", area1);
    println!("Percent difference in areas: {:e} %\n",
             screen1_area / area1 - 1.0);


    let screen2 = Screen::new(
        3.2, 320, 240
    );
    let screen2_area = screen2.area();
    println!(screen_print!(), screen2_area);
    let screen2_lengths = screen2.side_lengths();
    println!("screen2 lengths in inches: {}, {}",
             screen2_lengths.width,
             screen2_lengths.height
    );
    let area2 = screen2_lengths.area();
    println!("Area using side lengths: {}", area2);
    println!("Percent difference in areas: {:e} %\n",
             screen2_area / area2 - 1.0);


    let percent_diff = screen1_area / screen2_area - 1.0;
    println!(
        "Percent difference in areas of screens: {:.4e} %\n",
        percent_diff * 100.0
    );

    let numbers = vec![2.3, 4.5, -23_333.0123];
    //let numbers = vec![2, 4, -23333];
    println!("`numbers` = {:?}", numbers);

    let numbers_2 = math::sum_pow(&numbers, 5);
    println!("Fifth-order sum of `numbers` = {}", numbers_2);

    let numbers_3 = math::sum_pow(&[
            5.0_f64.sqrt(),
            5.0_f64.sqrt()], 2);
    println!("Sum of squares of `[sqrt(5.0), sqrt(5.0)]` = {}", numbers_3);

    let numbers_4 = math::mean(&numbers, 5);
    println!("Fifth-order mean of `numbers`, \
             the fifth-order sum divided by 3 = {}", numbers_4);

    let numbers_5 = math::rms(&numbers);
    println!("RMS of `numbers` = {}\n", numbers_5);
    //println!(debug_print!(numbers_5));

    let recfloat = RealRectangle::new(3245.234, 2344.2344);
    println!("recfloat = {:#?}", recfloat);

    let rec_ratio = recfloat.ratio();
    println!("rec_ratio = {}", rec_ratio);

    let rec_ratio2_the_refloatening = recfloat.width / recfloat.height;
    println!("rec_ratio2_the_refloatening = {}", rec_ratio2_the_refloatening);
    let rec_ratio3_floatvengeance = {
        *rec_ratio.numer() as f64 / *rec_ratio.denom() as f64
    };
    println!("rec_ratio3_floatvengeance = {}", rec_ratio3_floatvengeance);

    println!("Percent difference in sequels: {:e} %",
             rec_ratio2_the_refloatening / rec_ratio3_floatvengeance - 1.0);
}
