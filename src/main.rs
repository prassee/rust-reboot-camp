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
    // a1.balance(); calling on a closed thorws compiler errors

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
}
