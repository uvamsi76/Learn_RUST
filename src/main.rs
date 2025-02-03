// fn is_even(n: i32) -> bool {
//     if n%2==0 {
//         return true
//     }
//     return false
// }

// fn com_fib(n:i32) -> i32{
    
//     let mut first=0;
//     let mut second=1;
//     if n==1{
//         return 1;
//     }
//     if n==0 {
//         return 0
//     }
//     for _ in 0..n-2{
//         let temp=second;
//         second=second+first;
//         first=temp;
//     }
//     return second;
// }

mod util1;

fn get_len_str(sa:String) -> usize{
    sa.chars().count()
}

struct User{
    first_name:String,
    last_name:String,
    age:i32

}

// a class like implementation in rust using struct

struct Rect{
    width:i32,
    height:i32
}

impl Rect{
    //implementing a method inside a particular class in rust
    fn area(&self) ->i32 {
        self.width*self.height
    }
    // If we want to implement static method we can do it this way
    fn print_static_value() -> i32{
        return 2
    }
}

fn main() {
    let s=String::from("Hello World!");
    let length = get_len_str(s);
    let user=User{
        first_name: String::from("testing"),
        last_name: String::from("Last name"),
        age: 32
    };
    let rectangle=Rect{
        width:32,
        height:32
    };

    println!("Area of the rectangle is {}",rectangle.area());
    println!("printing from the static method{}",Rect::print_static_value());
    println!("testing from differentfile {}",util1::testing_stuff_from_diff_file());
    println!(" ");
    println!("username is {}",user.first_name);
    println!("username lastname is {}",user.last_name);
    println!("username age is {}",user.age);
    println!("The number is {}",length);
}
