fn main() {
    println!("Hello, world!");
    let s = String::from("New string");
    println!("{}", first_word(&s));

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");

    let hello = &s[0..5];
    // let hello = &s[..5]; can also be written as 
    // drop the zero for cleaner syntax
    let world = &s[6..11];

    println!("{} {}", hello, world);
}

fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}