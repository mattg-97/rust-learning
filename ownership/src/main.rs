fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("string");
    let s2 = s1;
    //s1 is now invalid as it has been shallow copied
    //this means s2 now includes the pointer and length/capacity data from s1
    //this is the automatic safety of rust
    println!("{}", s2); //if i were to print s1 here it would throw an ownership error after move

    let s3 = String::from("new string");
    let s4 = s3.clone();
    //this is a deep copy meaning it copies all the allocated data from the heap
    //this is expensive and not default rust behaviour

    println!("{}", s3);
    println!("{}", s4);

    //this works differently with things like integers, this code is valid
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    //this is because x and y have a known size at compile time and are stored on the stack

    let s = String::from("String boy");

    takes_ownership(s);

    //s cant be used anymore, see funciton to see why

    let x = 5;

    makes_copy(x);

    println!("{} can still be used", x);

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    // IMPORTANT!!! The problem with all this is that giving and taking ownership all the time
    // is annoying and tedious, therefore we can use referencing and borrowing,
    // see the referencing folder for more
}

fn takes_ownership(some_string: String) {
    //some string is now in scope
    println!("{}", some_string); //it is then used and goes out of scope
} //then drop is called and memory is freed

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
} //doesnt go out of scope as it is a simple scalar type

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
