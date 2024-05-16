use tests; // remember this is just the name of this project
// NOTE: This only works because 'tests' is a lib and not a main. To test main, run it
mod common;

#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, tests::add_two(2));
}
