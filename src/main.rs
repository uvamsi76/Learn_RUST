use enums::{print_enum, shape};
mod optresenum;
use optresenum::find_first_a;
mod util1;
mod enums;

use std::fs::read_to_string;

fn main() {
    let ind=find_first_a(String::from("Preet"));
    
    // if ind==-1{
    //     println!("no a's buddy");
    // }
    // else{
    //     println!("a found at {}",ind);
    // }

    // match ind{
    //     Some(value) => println!("index is {}",value),
    //     None => println!("a not found")
    // }

    let result = read_to_string("a.txt");
    match result {
        Ok(data)=> println!("the data in the file is {}",data),
        Err(err)=> println!("Some error occured see the details of the error {}",err)
    };
}
