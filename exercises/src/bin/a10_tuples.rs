fn cartesian() -> (i32,i32){
    (4, 5)
}
pub fn main() {
    let (x,y) = cartesian();
    if y == 5{
        println!("The X coordinate is {}", x);
        println!("Y coordinate of {} is equal to 5", y);
    }
    else if y > 5{
        println!("The X coordinate is {}", x);
        println!("Y coordinate of {} is greater 5", y);
    }
    else if y < 5{
        println!("The X coordinate is {}", x);
        println!("Y coordinate of {} is less 5", y);
    }



    let coordinates = cartesian();
    if coordinates.1 == 5{
        println!("The X coordinate is {}", coordinates.0);
        println!("Y coordinate of {} is equal to 5", coordinates.1);
    }
    else if y > 5{
        println!("The X coordinate is {}", coordinates.0);
        println!("Y coordinate of {} is greater 5", coordinates.1);
    }
    else if y < 5{
        println!("The X coordinate is {}", coordinates.0);
        println!("Y coordinate of {} is less 5", coordinates.1);
    }
}
