// Closure works like making a simple variable into a callable function
pub fn main(){
    let add = |x:i8,y:i8| {
        println!("Returning some value: {} + {}", x, y);
        x + y
    };
    let result = add(13,19);
    let print_result = |x| println!("Total: {}", (result + x));
    print_result(27);

}
