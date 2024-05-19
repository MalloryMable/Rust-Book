use core::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

//TODO: Vec<(ShirtColor, u32)>
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    //TODO: in value version decrement shirt color. Should I make this a map?
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        //TODO: Just get largest by iterating through vector store high value and high shirt
        let mut red_count: u32 = 0;
        let mut blue_count: u32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // CLOSURES

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    // NOTE: debug print statement
    println!("The user preference {:?} gets {:?}", user_pref1, giveaway1);

    let giveaway2 = store.giveaway(None);
    println!("The user with no preference gets {:?}", giveaway2);

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("2 = {}", expensive_closure(2));
    // NOTE: All below are valid closure syntax 
    // fn add_one_v1 (x: u32) -> u32 { x + 1 }

    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1;
    
    // NOTE: Demonstration of closure and ownership
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    
    only_borrows();

    println!("Post static closure pre mutable closure {:?}", list);

    let mut mutable_borrow = || list.push(4);
    

    mutable_borrow();
    println!("After calling closure: {:?}", list);

    // FnMut demonstration
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // ITERATOR
    
    // Gets all rectangles under a certain height using iter
    
    //first we make a non mutable iterator for deomostration purposes
    let list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    println!("{:#?}", list.iter()
        .filter(|x| x.height < 10)
        .collect::<Vec<_>>()
        );

}


