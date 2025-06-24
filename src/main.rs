use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    let mut guess = String::new();
    loop {
        println!("Please input your guess.");

        guess.clear();

        // The following is a more consice way to handle the error than using the match as in the followed comment
        // since the default arm in the match doesn't do anything.
        if let Err(e) = io::stdin()
            .read_line(&mut guess) {
                println!("{e}")
            }

        // match  io::stdin()
        //     .read_line(&mut guess){
        //     Err(e) => {
        //         println!("{e}" )
        //     },
        //     _ => {}
        // }
        

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Your guess should be an integer");
                continue;
            }
        };

        println!("You guessed: {guess}");
        // .expect("Please type a number!");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    

}