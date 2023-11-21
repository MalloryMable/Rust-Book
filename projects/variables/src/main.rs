fn main() {
    //shadowing
    let x = 5;

    let x = x + 1;
    println!{"{x}"};

    {
        let x = x * 2;
        println!{"The value of x in the inner scope is: {x}"};
    }

    println!{"The value of x is: {x}"};

    //tupples
    let x: (i32, f64, u8) = ( 500, 6.4, 1 );

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Tupple:({five_hundred}, {six_point_four}, {one})");

    //arrays
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    println!("a[2]: {}", a[2]); 
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [u8; 5] = [3; 5];
    println!("months[8]: {}, a[0]: {}, a[2]: {}",months[8], a[0], a[2]);

}

