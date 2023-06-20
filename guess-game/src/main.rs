use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");
    let sys_guess = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("please input number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your number is :{guess}");

        match guess.cmp(&sys_guess) {
            Ordering::Greater => println!("too big"),
            Ordering::Less    => println!("too less"),
            Ordering::Equal   => {
                println!("you win");
                break;
            },

        }
    }
}
