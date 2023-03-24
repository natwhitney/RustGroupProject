/*
Nathan Whitney
3/24/23

Simple Fizz Buzz implementation program. counts from 1 to 100.
Prints Fizz if number is divisible by 3, prints Buzz if number is divisible by 5
Prints FizzBuzz if number is divisible by both 3 and 5
*/

fn main() {
    for num in 1..100 {
        if (num % 3 == 0) & (num % 5 == 0){
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
    }
}
