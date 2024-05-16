pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value)
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value)
        } // since panic ends the function else is unneeded, but else is more clear

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    // rotated out
    format!("Hello {}!", name)
    
    // String::from("Hello world!")
}

#[cfg(test)]
mod tests {
    // NOTE: this must be used to make use of all parent modules
    use super::*;
    
    // Guess() tests
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Guess struct tests
    #[test]
    fn guess_confirm() {
        let guess = Guess::new(59);
        assert_eq!(59, guess.value);
    }

    // Practice with assert_eq!()
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    
    // Practice with panic!() and failed tests
    #[test]
    #[should_panic]
    fn failed(){
        panic!("This test was designed to fail");
    }
    
    // add() test, 
    #[test]
    fn it_works() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
   
    // add_two() test
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    // greeting() test
    #[test]
    fn greeting_cointains_name() {
        let result = greeting("Carol");
        // NOTE: output is natively captured run test with '-- --show-output' otherwise
        println!("{}", result);
        assert!(result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result);
    }


    // NOTE: only run when '-- --ignored' or '-- --ignore-included' is called
    // NOT actually fibonacci just big
    #[test]
    #[ignore]
    fn slow_test() {
        let mut fib: u64 = 0;
        for i in 1..2u64.pow(26) {
            fib += i;
            println!("{}", fib);
        }

    }
}
