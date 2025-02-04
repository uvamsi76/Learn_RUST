//enums


//options enum which we use this for null handling
pub fn find_first_a(s : String) -> Option<i32>{
    for (index,cha) in s.chars().enumerate(){
        if cha=='a' {
            return Some(index as i32);
        }
    }
    return None;
}

// Result enum we use this for error handling

// we are doing this in main itself by reading a file