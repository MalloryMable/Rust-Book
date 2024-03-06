use std::fs::File;

fn main() {
    /* While this also panics it can be helpful to understand the case
     * that was assumed to always be true */
    let greeting = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    panic!("HAC!");
}
