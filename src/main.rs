use enums::{print_enum, shape};

mod util1;
mod enums;


fn main() {
    let s=String::from("Hello World!");
    let length = util1::get_len_str(s);
    let user=util1::User{
        first_name: String::from("testing"),
        last_name: String::from("Last name"),
        age: 32
    };
    let rectangle=util1::Rect{
        width:32,
        height:32
    };

    println!("Area of the rectangle is {}",rectangle.area());
    println!("printing from the static method{}",util1::Rect::print_static_value());
    println!("testing from differentfile {}",util1::testing_stuff_from_diff_file());
    let shape=shape::square(3.0);
    print_enum(shape);
    // let dir=enums::Direction::North;
    // println!("this is a enum {}", dir);
    println!(" ");
    println!("username is {}",user.first_name);
    println!("username lastname is {}",user.last_name);
    println!("username age is {}",user.age);
    println!("The number is {}",length);
}
