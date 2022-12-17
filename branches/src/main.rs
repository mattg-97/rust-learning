fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    if number != 0 {
        println!("This number is something other than 0!");
    }

    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("the value of x is {x}");
}
