fn main() {
    let some_number = Some(5);
    let some_char = Some('c');

    let absent_number: Option<i32> = None;

    let x: i8 = 6;
    let y: Option<i8> = Some(5); // options are rusts way of handling null
    let sum = x + y.unwrap(); //unwwrap gets the option value
    println!("Hello, world! {}", sum);
    
    let c = value_in_cents(Coin::Quarter(States::Wyoming));

    let five = Some(5);
    let six = Some(6);
    let none = plus_one(None);

    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("the maximum is configured to be {}.", max);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(States)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum States {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    IllinoisIndiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming
}
