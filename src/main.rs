mod level_01_fundementals;
use level_01_fundementals::drill_1::to_fahrenheit;
use level_01_fundementals::drill_2::largest;
use level_01_fundementals::drill_2_recall::smallest;
use level_01_fundementals::drill_3::{reverse, vec_reverse};
use level_01_fundementals::drill_4::divide;
use level_01_fundementals::drill_5::get_code_label;
use level_01_fundementals::drill_6::{classify, describe, double_first};
fn main() {
    println!("convert to to_fahrenheit {:?}", to_fahrenheit(100));
    println!("{:?}", largest(&[10, 23, 34]));
    println!("{:?}", smallest(&[10, 23, 34]));
    println!("{:?}", reverse("text"));
    println!("{:?}", vec_reverse("text"));
    println!("{:?}", divide(23, 5));
    println!("{:?}", get_code_label(200));
    println!("{:?}", classify(None));
    println!("{:?}", describe(None));
    println!("{:?}", double_first(&[]));
}
