fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //this & is a reference to the data in s1

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");

    change(&mut s);// we are making a mutable reference here
    // YOU CAN ONLY HAVE ONE REFERENCE TO A MUTABLE VALUE
    // however if we put it out of scope that is fine.

    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

}

fn calculate_length(s: &String) -> usize { //& indicates were taking a reference to a string
    // we cannot modify something we have a reference to, therefore if we tried to change the
    // value of s here the program wouldnt compile
    s.len()
}// s goes out of scope here, but because it is a reference it isnt dropped as it isnt 'owned'

fn change(some_string: &mut String) {
    some_string.push_srt(", world!"); //because the reference is mutable we can change the data we
    //are referencing without owning it
}

// reference dangling
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!