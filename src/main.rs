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

fn get_len_str(sa:String) -> usize{
    sa.chars().count()
}

struct User{
    first_name:String,
    last_name:String,
    age:i32

}
fn main() {
    let s=String::from("Hello World!");
    let length = get_len_str(s);
    let user=User{
        first_name: String::from("testing"),
        last_name: String::from("Last name"),
        age: 32
    };
    println!("username is {}",user.first_name);
    println!("username lastname is {}",user.last_name);
    println!("username age is {}",user.age);
    println!("The number is {}",length);
}
