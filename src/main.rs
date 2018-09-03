fn main() {
    for n in 1..101 {
        match n {
            _ if n % 15 == 0 => println!("FizzBuzz"),
            _ if n % 5 == 0 => println!("Buzz"),
            _ if n % 3 == 0 => println!("Fizz"),
            _ => println!("{}", n),
        }
    }
}
