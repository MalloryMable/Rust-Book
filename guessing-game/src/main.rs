use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number from 1 to 100.");

    let secret = rand::thread_rng().gen_range(1..=100);
    
    let mut score: u32 = 1;

    loop{ 
        println!("Please input your guess:");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number.");
 
        match guess.cmp(&secret){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
        score += 1;
    }
    println!("Your score {score}");
    if score <=6 {
        println!("Within binary search!");
    }
}

