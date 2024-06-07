struct Structure {
    string: Option<String>,
    integer: Option<usize>,
}

struct Point{
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Hello {id: i32},
}

fn main() {
    // Example control follow
    let structure = Structure {string: None, integer: Some(144) };

    if let Some( _ ) = structure.string {
        println!("That's weird there shouldn't be a string");
    } else if let Some(value) = structure.integer {
        match value {
            4 => println!("2 squared"),
            9 => println!("3 squared"),
            16 => println!("4(2 squared) squared"),
            144 => println!("12 squared [3 squared and 4(2 squared) squared]"),
            _ => println!("If you can't have a cool number why have one at all"),
        }
    } else {
        println!("Default Case!");
    }

    let enumeration = (3 as u32, 4 as i32, 12 as usize);

    // destructuring also exists in rust
    let (a , b, c) = enumeration;
    println!("{} * {} = {} thus {} * {} = {}", a, b, c, a.pow(2), b.pow(2), c.pow(2));

    // Multiple matches per arm
    let x = 'm';
    let y = 12;

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Strange..."),
    }

    match y {
        2..=4 | 6 | 12 | 24 | 36 | 48 | 72 => println!("This number could go into 144"),
        _ => println!("Try a cooler number :/"),
    }

    // Destructuring
    let point = Point{x: 11, y: 13, z: 144};
    let msg = Message::ChangeColor(0, 144, 255);

    // Enum destructure
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
    Message::Hello { id:_ } => println!("Sure I guess"), 
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {r}, green: {g}, blue: {b}",);
        }
    }

    let msg = Message::Hello {id: 5};

    // NOTE: Actually new! @ opperator
    // The @ opperator lets you do that c thing where you declare and use a variable at
    // the same time

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        
         Message::Hello { 
            id: id_var @ 3..=7 
        } => println!("Found id in range {id_var}"),
        Message::Hello { id: 10..=12 } => println!("Found an id in another string"),
        Message::Hello { id } => println!("Found some other id: {}", id),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {r}, green: {g}, blue: {b}",);
        }
    }

    // Struct destructure
    match point {
        Point {x: 0, y: 0, ..} => println!("On the origin!"),
        Point {x, y: 0, ..} => println!("On the x axis at {x}"),
        Point {x: 0, y, ..} => println!("On the y axis at {y}"),
        Point {x, y, ..} => println!("On neither axis: ({x}, {y})"),
    }

    let x = 16;
    let y = false;

    match x {
        16 if y => println!("yes!"),
        _ => println!("no"),
    }


}
