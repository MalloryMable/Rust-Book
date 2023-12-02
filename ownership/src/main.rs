fn main() {
    //String slices multiple slices from the same space allowable because & is not mut
    let s: String = String::from("Hello, world!");
    
    let hello = &s[..5]; //starts at 0
    let world = &s[6..12]; //one less than the final char to cut off '!'

    let full_slice = &s[..]; //starts at 0 to last index
    println!("hello = {hello}, world = {world}, full_slice = {full_slice} firstword(s) = {}", first_word(&s));

    //array slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]); //this is comparing the pointer from (a[1], a[2]) to (a[2], a[3])
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

   &s[..]
}
