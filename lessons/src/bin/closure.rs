// Closure works like making a simple variable into a callable function
pub fn main() {
    let add = |x: i8, y: i8| {
        println!("Returning some value: {} + {}", x, y);
        x + y
    };
    let result = add(13, 19);
    let print_result = |x| println!("Total: {}", (result + x));
    print_result(27);

    struct Fullname {
        first_name: String,
        last_name: String,
    }
    let mut name = Fullname {
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
    };
    println!("Old name: {} {}", name.first_name, name.last_name);
    let mut new_name = |new_first_name: &str| {
        name.first_name = new_first_name.to_string();
        name.last_name = "Warhammer".to_string();
    };
    new_name("John");
    println!("New name: {} {}", name.first_name, name.last_name);
}
