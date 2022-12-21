use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 50);

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //scores.get returns an option
    //.copied makes sure its an option and not an &option
    //unwrap_or means it either unwraps the option or returns 0 if it cant be found
    println!("{score}");

    for (key, value) in &scores {
        println!("Key:{} Value:{}", key, value);
    }

    //overwriting a value
    let mut scores2 = HashMap::new();
    scores2.insert("Blue".to_string(), 10);
    scores2.insert("Blue".to_string(), 50);
    println!("{:?}", scores2);

    //adding a key and value only if key isnt present
    let mut scores3 = HashMap::new();
    scores3.insert("Blue".to_string(), 10);
    scores3.entry("Yellow".to_string()).or_insert(30);
    scores3.entry("Blue".to_string()).or_insert(30); //key is present so doesnt insert
    println!("{:?}", scores3);

    

}
