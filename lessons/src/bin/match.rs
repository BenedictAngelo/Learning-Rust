fn main() {
    let some_bool = true;
    match some_bool{
        true => println!("It is true"),
        false => println!("It is false"),
    };

    let match_numbers: u16 = 1;
    match match_numbers{
        150..=200 => {
            println!("It is 150 and up 200")
        },
        _ => {
            println!("Not included");
        }
    };

    let match_strings: &str = "America";
    match match_strings {
        "Germany" => {
            println!("It is Germany")
        },
        _ => {
            println!("Other countries");
        }
    };

    let match_array = [100, 200,250,400];
    match match_array[0..=3]{
        [100,150] => {
            println!("It is 100 and above");
        },
        [100,200,..] => {
            println!("There is much more");
        },
        _ => {
            println!("A number is not included in the list");
        }
    }

}
