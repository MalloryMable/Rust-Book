#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        //height.checked_mul(width) is what I would do if I know how Option worked
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale: u32 = 2;
    let rect1 = Rectangle {
        width: scale * 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 0,
        height: 42,

    }; 
   
    // what I'd do if I knew how throwing things worked. Using DeMorgans
    if !(rect1.width() && rect2.width())   {
        println!("I would have thrown an error here");
    } 

    let rect2 = Rectangle {
        width: 23,
        ..rect2
    };


    //debug print for a passed object pointer 
    dbg!(&rect2);
    println!("The width exists: {}", &rect1.width());
    println!("rect1(area: {}) can hold rect2(area: {})?\n{}", rect1.area(), rect2.area(), rect1.can_hold(&rect2));
}

