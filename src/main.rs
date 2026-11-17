mod level_01_fundementals;
use level_01_fundementals::drill_1::to_fahrenheit;
use level_01_fundementals::drill_2::largest;
use level_01_fundementals::drill_2_recall::smallest;
use level_01_fundementals::drill_3::{reverse, vec_reverse};
use level_01_fundementals::drill_4::divide;
use level_01_fundementals::drill_5::get_code_label;
use level_01_fundementals::drill_6::{classify, describe, double_first};
use level_01_fundementals::drill_7::{first_negative, next_power_of_two, parse_and_double};
mod level_02_structs_enums;
use level_02_structs_enums::drill_1::*;
use level_02_structs_enums::drill_2::*;
use level_02_structs_enums::drill_3::*;
use level_02_structs_enums::drill_4::{Point, Shape};
use level_02_structs_enums::drill_5;
use level_02_structs_enums::drill_6;
mod level_03_ownership;
use level_03_ownership::{drill_1, drill_3, drill_4};
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
    println!("{:?}", first_negative(&[[1, 1, 1], [2, 2, 2]]));
    println!("{:?}", parse_and_double(" 23 "));
    println!("{:?}", next_power_of_two(23));

    let server_config = ServerConfig::new("localhost".to_string(), 3306);
    let backup_server = server_config.with_port(3307);
    println!("{:?}", server_config);
    println!("{:?}", backup_server);
    println!("{:?}", HealthCheck);
    println!("{:?}", Meters(30.0));

    let mut a1 = Account::new("ping".to_string());
    println!("{:?}", a1.owner());
    println!("{:?}", a1.balance());
    a1.deposit(100);
    println!("{:?}", a1.balance());
    match a1.withdraw(50) {
        Ok(()) => println!("{:?}", a1.balance()),
        Err(e) => println!("{}", e),
    }
    match a1.withdraw(50) {
        Ok(()) => println!("{:?}", a1.balance()),
        Err(e) => println!("{}", e),
    }
    a1.deposit(50);
    let closing_bal = a1.close();
    println!("{:?}", closing_bal);
    // a1.balance(); calling on a closed throws compiler errors

    let event = Event::Close;
    println!("{:?}", event);
    println!("{:?}", Event::Click { x: 23, y: 43 });
    println!("{:?}", Event::KeyPress('a'));
    println!("{:?}", Event::Paste("abcd".to_string()));
    println!("{:?}", Event::Resize(23, 45));
    let click_desc = Event::Click { x: 23, y: 43 }.describe();
    println!("{:?}", click_desc);
    let resize_desc = Event::Resize(23, 67).describe();
    println!("{:?}", resize_desc);
    let resize_typed_chars = Event::Resize(23, 67).typed_chars();
    println!("{:?}", resize_typed_chars);

    let point = Point::new(2, 3);
    println!("{}", point);

    let circle = Shape::Circle {
        center: point,
        radius: 23,
    };
    let rect = Shape::Rect {
        top_left: Point::new(7, 9),
        width: 23,
        height: 45,
    };
    println!("{}", circle);
    println!("{}", rect);

    let shape1 = drill_5::Shape::Circle {
        center: drill_5::Point::new(0, 0),
        radius: 2,
    };
    let line = drill_5::Shape::Line(drill_5::Point::new(0, 0), drill_5::Point::new(0, 0));
    drill_5::starts_at_origin(&shape1);
    drill_5::vertical_x(&line);
    drill_5::mirror(drill_5::Point::new(0, 3));

    let d6_tempr = drill_6::Celsius(23.3);
    println!(
        "{:?} , {:?}",
        d6_tempr,
        d6_tempr == drill_6::Celsius(23.3000)
    );

    let d6_usid = drill_6::UserId(23);
    println!("{:?} {:?}", d6_usid.0, d6_usid == drill_6::UserId(23));
    println!("{:?}", d6_usid);

    let drill6_settings = drill_6::Settings {
        theme: "ping".to_string(),
        font_size: 2,
        autosave: false,
    };
    let drill6_new_settings = drill_6::Settings {
        autosave: true,
        ..drill6_settings
    };
    println!("{:?}", drill6_new_settings);
    // println!("{:?}", drill6_settings.theme);
    // println!("{:?}", drill6_settings);

    drill_1::borrow_str("s");
    drill_1::take_string("s".to_string());
    drill_1::take_number(23);
    drill_1::take_tuple((23, "abcd".to_string()));
    drill_1::take_and_return("s".to_string());

    drill_3::double_all(&mut [1, 2, 3]);
    drill_3::zero_first(&mut [1, 2, 3]);
    drill_3::append_total(&mut vec![1, 2, 3]);

    drill_4::first_word("s");
    drill_4::middle(&[1, 2, 3, 4]);

    println!("{}", rust_curious::basic_syntax::factorial(10));
    println!("{}", rust_curious::basic_syntax::factorial_loop(10));
}

mod rust_curious {
    pub mod basic_syntax {
        pub fn factorial(n: u64) -> u64 {
            if n == 0 || n == 1 {
                1
            } else {
                n * factorial(n - 1)
            }
        }

        pub fn factorial_loop(n: u64) -> u64 {
            let mut fact = 1;
            let mut x = 1;
            loop {
                fact *= x;
                if x == n {
                    break;
                } else {
                    x += 1;
                }
            }
            fact
        }
    }
}
