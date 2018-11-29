extern crate num_rational;
extern crate num_traits;
#[macro_use] extern crate fomat_macros;
mod shapes;
mod math;

use shapes::rectangle::RealRectangle;
use shapes::screen::Screen;
use shapes::ratio::HasRatio;

use math::percent_diff;

macro_rules! area_print {
    // $ord, $name, and $unit are string literals, $val is a number
    ($ord:tt, $name: tt, $val:ident, $unit:tt) => {
        pintln!("The area of the " $ord " " $name
                " is " ($val) " square " $unit ".")
    }
}

fn main() {
    let screen1 = Screen::new(7.0, 800, 480);
    let screen1_area = screen1.area();
    let screen1_lengths = screen1.side_lengths();
    pintln!((=screen1_lengths.width)" inches,\n"
            (=screen1_lengths.height)" inches");
    area_print!("first", "screen", screen1_area, "inches");
    println!();

    let screen2 = Screen::new(3.2, 320, 240);
    let screen2_area = screen2.area();
    let screen2_lengths = screen2.side_lengths();
    pintln!((=screen2_lengths.width)" inches,\n"
            (=screen2_lengths.height)" inches");
    area_print!("second", "screen", screen2_area, "inches");
    println!();

    pintln!("Percent difference in areas of screens: "
            {(percent_diff(screen1_area, screen2_area)):.4}" %\n\n");


    let numbers = vec![2.3, 4.5, -23_333.0123];
    //let numbers = vec![2, 4, -23333];
    pintln!([=numbers]"\n");

    let numbers_2 = math::sum_pow(&numbers, 5);
    pintln!("Fifth-order sum of `numbers` = "(numbers_2));

    let numbers_3 = math::sum_pow(&[
            5.0_f64.sqrt(),
            5.0_f64.sqrt()], 255);
    pintln!("255th-order sum of `[sqrt(5.0), sqrt(5.0)]` = "{numbers_3:e});

    let numbers_4 = math::mean(&numbers, 5);
    pintln!("Fifth-order mean of `numbers` (fifth-order sum divided by 3) = "
            (numbers_4));

    let numbers_5 = math::rms(&numbers);
    pintln!("RMS of `numbers` = "(numbers_5)"\n\n");
    //println!(debug_print!(numbers_5));

    let recfloat = RealRectangle::new(3245.234, 2344.2344);
    pintln!([=recfloat] "\n");

    println!(
        "Demonstrating use of Ratio<T>::approximate_float() to rationalize\n\
        a float division, then dividing the ratio to reconstruct the float.\n"
    );

    let rec_ratio = recfloat.ratio();
    pintln!("    "(=rec_ratio));

    let rec_ratio2_the_refloatening = recfloat.width / recfloat.height;
    pintln!("    "(=rec_ratio2_the_refloatening));
    let rec_ratio3_floatvengeance = {
        *rec_ratio.numer() as f64 / *rec_ratio.denom() as f64
    };
    pintln!("    "(=rec_ratio3_floatvengeance));

    pintln!("    Percent difference in sequels: "
            {(percent_diff(rec_ratio2_the_refloatening,
                           rec_ratio3_floatvengeance)):e}
            " %");
}
