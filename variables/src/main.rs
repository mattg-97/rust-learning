fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let x = 2.0;
    let y: f32 = 3.0;

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of the deconstructed tuple x is: {x}");
    println!("The value of the deconstructed tuple y is: {y}");
    println!("The value of the deconstructed tuple z is: {z}");
    println!("The value of the deconstructed tuple x is: {}", tup.0);
    println!("The value of the deconstructed tuple y is: {}", tup.1);
    println!("The value of the deconstructed tuple z is: {}", tup.2);


    let a = [1, 2, 3, 4, 5, 6, 7]; //normal array 
    let a: [i32; 7] = [1, 2, 3, 4, 5, 6, 7]; //array with type annotation

    println!("The first array value is: {}", a[0]);
    another_function();
}

fn another_function() {
    println!("Another function!");
}
