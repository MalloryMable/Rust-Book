use rand::Rng;

fn main() {
    //terinary opperator
    let condition = true;
    let number = if condition {1} else {0};

    println!("The value of number is: {number}");

    //while rand boolean is true
    println!("Go!");
    let mut rng = rand::thread_rng();

    while rng.gen() { //NOTE: best practices to gen::<bool>() as seen below
        println!("again!");
    }

    loop{
        if !rng.gen::<bool>() {
            break;
        }
        println!("again.");
    }

    while !rng.gen_bool(0.1) { //Argument for .gen_bool is percent chance of false
        println!("again...");
    }

    //control flow and returning values from loops
    let mut counter: u8 = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Twice counter = 10 is {result}");

    //enhance for loops: arrays
    let a = [10, 20, 30, 40];
    let mut counter: u8 = 1;
    for element in a {
        println!("the value a[{counter}]: {element}");
        counter += 1;
    }

    //enhanced for loop of a reversed range (iterable not an array like python)
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!!");


    //FUNCTIONS

    //set variable from function
    let x = five(); 
   
    println!("The value of x is: {x}");
    
    //set variable from code snippet
    let y = {
        let x =3;
        x + 1
    };
    
    println!("The value of y is: {y}");

    //print data with function
    another_function(5, 'h');

    //get variable from function with input
    let x = plus_one(x); //reused old x before shadowing

    println!("The value of x is: {x}");

    //Degree conversion:
    let temp: i16 = rng.gen_range(-50..250);
    println!("F: {temp} to C {}", f_to_c(temp));

    //fibonacci number
    let n:u8 = rng.gen_range(1..92); //92 overflows u64 because exponential gross is very strong
    println!("Fib n[{}]: {}", n, fib(n));

    
    //hard coding these in rather than adding a speical case for loop one

    println!("On the first day of Christmas,");
    println!("My true love sent to me");
    println!("A partridge in a pair tree.");
    for i in 2..13 {
        christmas_lyrics(i)
    }
}

fn another_function(value: u32, unit_label: char){
    println!("The measurment is: {value}{unit_label}");
}

//both take advantage of non semicolon returning
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn f_to_c(f: i16) -> i16 {
    let temp:i16 = (f-32)*5;
    if temp % 9 >= 5 {
        temp / 9 + 1
    } else {
        temp / 9
    }
}

fn fib(n: u8) -> u64 {
    let mut current: u64 = 1;
    let mut prev: u64 = 0;
    for _i in 0..=n {
        ( prev, current ) = (current, prev + current);
    }
    prev
}

fn christmas_lyrics(day: u8) {
    let days = ["And a", "Two", "Three", "Four", "Five", "Six",
        "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
    let gift = ["partridge in a pair tree.", "turtle doves,",
        "french hens,", "calling birds,", "golden rings,",
        "geese a-laying,", "swans a-swimming,", "maids a-milking",
        "ladies dancing", "lords a-laying", "pipers piping,",
        "drummers drumming,"];

    println!();
    println!("On the {} day of Christmas", number_to_ordinal(day));
    println!("My true love sent to me");
    for i in (0..day).rev(){
        println!("{} {}", days[i as usize], gift[i as usize]);
    }
}

fn number_to_ordinal(num: u8) -> &'static str {
    match num {
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "unknown", // Default case for numbers outside 2-12
    }
}
