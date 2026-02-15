struct Grocery{
    quantity: i32,
    id_number: i32,
}
fn show_quantity(grocery:&Grocery){
    println!("There are {} stocks", grocery.quantity);
}
fn show_id_number(grocery:&Grocery){
    println!("{} is the ID of the item", grocery.id_number);
}
pub fn main() {
    enum Items{
        Lettuce,
        Spinach,
        Carrot,
    }
    let item = Items::Lettuce;
    let groceries = Grocery {
        quantity: 10,
        id_number: 16,
    };
    show_quantity(&groceries);
    show_id_number(&groceries);
    match item{
        Items::Lettuce => println!("The item name is lettuce"),
        Items::Carrot => println!("The item name is Carrot"),
        Items::Spinach => println!("The item name is Spinach"),

    }
}
