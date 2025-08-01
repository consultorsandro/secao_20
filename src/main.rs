fn main() { // class 352
   let multiplier = 5;

   let multiply_by = |value| value * multiplier;
   println!("{}", multiply_by(3 as u8));

   let numbers = vec![1, 2, 3, 4, 5];
   println!("{:?}", numbers);

   let print_numbers = ||println!("{:?}", numbers);
   print_numbers();
   print_numbers();
   println!("{:?}", numbers);
}

/*
fn main() { // class 350
   let multiplier = 5;

   let multiply_by = |value| value * multiplier;
   println!("{}", multiply_by(3 as u8));

   let mirror = |value| value;
   println!("{}", mirror("why so serious?")); // Closure with different types
}
*/
