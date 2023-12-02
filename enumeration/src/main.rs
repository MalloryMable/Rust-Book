use rand::Rng;
#[derive(Debug)]
enum UsState { 
    Alabama,
    Alaska,
    Virginia,
    Nebraska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//example of impl function for enum
impl Coin {
    fn value_in_cents(&self) -> u8 {
        match &self {
           Coin::Penny => 1,
           Coin::Nickel => 5,
           Coin::Dime => 10,
           Coin::Quarter(state) => {
               println!("State quarter {:?} generated!", state);
               25
           }
        }
    }
}

fn main() {
    // example of naive if else match statement for Option<>
    fn print_coin_optional(bowl: &Option<Coin>) {
       match &bowl {  
           None => println!("Empty bowl"), 
           Some(extant_coin) => println!("Known coin: {:?}", extant_coin),
       }
    }

    //Option implementiation
    let mut bowl: Option<Coin> = None;

    //None case
    print_coin_optional(&bowl);

    bowl = Some(Coin::Quarter(UsState::Nebraska));
    print_coin_optional(&bowl);
    
    // Example of valeue being returned to some and underscore default
    let mut rng = rand::thread_rng();
    bowl = Some(
        match rng.gen_range(0..4){
            0 => Coin::Penny,
            1 => Coin::Nickel,
            2 => Coin::Dime,
            3 => Coin::Quarter(roll_state(&mut rng)),
            _ => unreachable!(),
        }
    );

    //if let notation coin variable destructured out of Some value pulled from bowl
    if let Some(coin) = bowl {
        println!("New coin({:?}) value: {}",coin, coin.value_in_cents());
    }

    fn roll_state(rng: &mut impl Rng) -> UsState {
        match rng.gen_range(0..50) {
            0 => UsState::Alaska,
            1 => UsState::Alabama,
            2 => UsState::Nebraska,
            3 => UsState::Virginia,
            _ => roll_state(rng),
        }
    }
}
