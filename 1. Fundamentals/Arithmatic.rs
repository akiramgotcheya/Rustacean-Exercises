# Question 1
// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// Solution

fn main() {
   
    let (x, y) = (1, 2);
    let s = sum (x, y);
    println!("{:?}",s);
}

fn sum(x:i32, y:i32) -> i32  {
    x + y
}
