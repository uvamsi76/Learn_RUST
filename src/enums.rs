//simple enums
pub enum Direction{
    North,
    South,
    East,
    West
}

// enums with associated values
pub enum shape{
    circle(f32),
    square(f32),
    rectangle(f32,f32)
}

pub fn print_enum(shape:shape) -> f32 {
    
    // pattern matching
    let area=match shape {
        shape::circle(r) => 3.14*r*r,
        shape::square(a) => a*a,
        shape::rectangle(a,b ) => a*b 
    };
    println!("Area is {}",area);
    return area;
}
