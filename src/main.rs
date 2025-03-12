use std::io;
fn main() {
    let random_number = rand::random_range(0..=100);
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if guess > random_number {
            println!("Guess a lower number next time!");
        }
        else if guess < random_number {
            println!("Guess a higher number next time!");
        }
        else {
            println!("You guessed the right number!");
            break;
        }
    }
}
