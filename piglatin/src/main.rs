fn main() {
    let mut string_to_convert = "hello".to_string();
    convert_to_pig_latin(&mut string_to_convert);
    println!("{}", string_to_convert);
    let string_to_convert_two = "cheese".to_string();
    let converted_string = convert_to_pig_latin_with_return(string_to_convert_two);
    println!("{}", converted_string);
}

fn convert_to_pig_latin(string_to_convert: &mut String) {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let first_char = string_to_convert.chars().next().unwrap();
    if VOWELS.contains(&first_char){
        string_to_convert.push_str("-hay");
    } else {
        string_to_convert.remove(0);
        string_to_convert.push('-');
        string_to_convert.push(first_char);
        string_to_convert.push_str("ay");
    }
}

fn convert_to_pig_latin_with_return(mut string_to_convert: String) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let first_char = string_to_convert.chars().next().unwrap();
    if VOWELS.contains(&first_char){
        string_to_convert.push_str("-hay");
    } else {
        string_to_convert.remove(0);
        string_to_convert.push('-');
        string_to_convert.push(first_char);
        string_to_convert.push_str("ay");
    }
    string_to_convert
}

