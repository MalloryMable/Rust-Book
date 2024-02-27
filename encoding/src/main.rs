use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};

fn main() {

    // Local object for generating random number
    let mut rng = thread_rng();

    // Vecotors

    // Set vector length
    let n = rng.gen_range(1..=100);
    
    // Generating and mutating a vector
    let random_range: Vec<i8> = (0..n).map(|_| rng.gen_range(-128..=127)).collect();
    let sum: i16 = random_range.iter().map(|&x| x as i16).sum();
    let mean: f32 = sum as f32 / n as f32;
    println!("Multi line mean: {mean}");

    // All this mutation can be done on one line(2 if array size gets counted)
    let mean_one_liner: f32 = (0..n).map(|_| rng.gen_range(-128..=127) as f32).sum::<f32>() / n as f32;
    println!("One line mean: {mean_one_liner}");

    // Generating random text without a LLM is hard :(
    let s_prime = "I am a lesbian woman coding with rust is fun"; //Not nandling punctuation or multiple lines
    let s = String::from(s_prime).to_lowercase();

    let mut s_pig = String::new();
    for word in s.split_whitespace(){
        let first = &word[..1];
        
        // Also not handling two letter cases. Could check the first to chars. Don't wanna
        s_pig = if !matches!(first, "a" | "e" | "i" | "o" | "u" | "y"){
            format!("{}{}{first}ay ",s_pig, &word[1..])
        } else {
            format!("{}{word}yay ", s_pig)
        }
    }
    println!("\"{s}\" in pig latin is:\n{s_pig}");

    // Build a hash map to contain a simple org chart 
    let mut org: HashMap<String, Vec<String>> = HashMap::new();

    // This would be better implemented as a cli of some sort
    let instruct = vec![String::from("Add Amir to Sales"), String::from("Add Sally to Engineering"), String::from("Add Misha to Sales")];

    //let instruct = vec!["Add Amir to Sales", "Add Sally to Engineering", "Add Misha to Sales"];

    for command in instruct {
        let words: Vec<&str> = command.split_whitespace().collect();
        // another hasmap for employees is not necessary, but employees must be a vec
        let dep = org.entry(String::from(words[3])).or_insert(HashSet::<String>::new());
        // i think i could make this faster by making it a hashset
        let employee: String = String::from(words[1]);
        if !dep.contains(&employee){
            dep.insert(employee);
        }
    }

    println!( "Org chart: {:?}", org);
}
