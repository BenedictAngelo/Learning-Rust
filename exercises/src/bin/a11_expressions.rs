//ideal
fn print_message(gt_100: bool){
    match gt_100{
        true => println!("its big"),
        false => println!("its small"),
    }
}
//self made
fn say(){
    let moneys = 50;
    let money = match moneys > 100 {
        true => println!("It is big"),
        false => println!("It is small"),
    };
    return money;
}
pub fn main() {
    //self made
    say();
    //ideal
    let value = 100;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}
