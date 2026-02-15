#[derive(Debug)]
enum Colors{
    Red,
    Yellow,
    Blue,
}
struct Dimensions{
    height:i32,
    width:i32,
    length:i32,
}
struct Box{
    dimensions:Dimensions,
    weight: i32,
    color: Colors,
}
impl Box {
    fn red_box()->Self{
        Self {
            color:Colors::Red,
            dimensions:Dimensions{
                height:6,
                width:5,
                length:11,
            },
            weight: 100,
        }
    }
    fn yellow_box()->Self{
        Self {
            color:Colors::Yellow,
            dimensions:Dimensions{
                height:5,
                width:4,
                length:10,
            },
            weight: 110,
        }
    }
    fn blue_box()->Self{
        Self {
            color:Colors::Blue,
            dimensions:Dimensions{
                height:7,
                width:4,
                length:9,
            },
            weight: 90,
        }
    }
    fn display_box(&self){
        println!("{:?} shipping box with a {:?} length, {:?} width, {:?} height, {:?} weight",
            self.color,self.dimensions.length,self.dimensions.width,self.dimensions.height,self.weight);
    }
    
}
pub fn main() {
    let box_1 = Box::red_box();
    box_1.display_box();
    let box_2 = Box::yellow_box();
    box_2.display_box();
    let box_3 = Box::blue_box();
    box_3.display_box();
}
