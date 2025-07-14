use std::iter::Product;

fn main() {
   let multiplier = 5;

   let multiply_by = |value: i32| -> i32 {
       return value * multiplier;
   };

    println!("{}", multiply_by(2));

    let Product = | a: i32, b: i32 | -> i32 {
        println!("Calculating product for you");
        return a * b;
    };
    println!("{}", Product(3, 10));
    println!("{}", Product(5, 8));

}
