fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

   let mut number = 3;
   
   while number != 0 {
    println!("{number}");
    number -= 1;
   }
   println!("LISTOFF!");

   let a = [1, 2, 3, 4, 5, 6, 7];

   for element in a {
    println!("the value is: {element}");
   }

   for number in (1..4).rev() {
    println!("{number}");
   }
   println!("LISTOFF!");
}
