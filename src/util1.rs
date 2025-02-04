pub fn testing_stuff_from_diff_file() -> i32{
    return -1;
}

pub fn get_len_str(sa:String) -> usize{
    sa.chars().count()
}

pub struct User{
    pub first_name:String,
    pub last_name:String,
    pub age:i32

}

// a class like implementation in rust using struct

pub struct Rect{
    pub width:i32,
    pub height:i32
}

impl Rect{
    //implementing a method inside a particular class in rust
    pub fn area(&self) ->i32 {
        self.width*self.height
    }
    // If we want to implement static method we can do it this way
    pub fn print_static_value() -> i32{
        return 2
    }
}


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
