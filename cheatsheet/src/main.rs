//how enums and structs really work, its like a data contract
//means certain states will be unable to be represented
enum Living {
    Alive,
    Dead,
}

enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

struct Human {
    name: String,
    state: Living,
    home: Planet,
}
fn main() {
    // this means creating a human struct only allows for a certain representation
    let user = Human {
        name: "Matt".to_string(),
        state: Living::Alive,
        home: Planet::Earth,
    };

    // or

    let deadUser = Human {
        name: "Greg".to_string(),
        state: Living::Dead,
        home: Planet::Venus,
    };
}
