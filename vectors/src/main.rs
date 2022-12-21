fn main() {
    let _v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    v2.push(5);
    v2.push(6);
    v2.push(7);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    let third_option: Option<&i32> = v2.get(210);

    match third_option {
        Some(third_option) => println!("The third element is {third_option}!"),
        None => println!("There is no third element!")
    }
     

    for i in &v2 {
        println!("{}", i);
    }
}
