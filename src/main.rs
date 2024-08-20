use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MAX_WINS: u32 = 3;
const MAX_GUESSES: u32 = 5;

fn main() {
    let mut quit = false;
    let mut wins = 0;

    loop {
        if wins == MAX_WINS {
            println!("Max wins reached! Congrats!");
            break;
        } else if quit == true {
            println!("Thanks for playing!");
            break;
        }

        println!("Guess the number");
        println!("Current wins: {wins} | Target: {MAX_WINS}");
        
        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut total_guesses = 0;

        let mut auto = false;

        loop {
            println!("Please input your guess:");

            let mut guess = String::new();

            if !auto {
                io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            } else {
                guess = rand::thread_rng().gen_range(1..=100).to_string();
                auto = false;
            }

            let guess = guess.trim();

            if guess.to_lowercase() == "quit" {
                quit = true;
                break;
            }

            let guess: u32 = match guess.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };

            total_guesses += 1;

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small");
                    check_guesses(total_guesses, secret_number, &mut auto);
                },
                Ordering::Greater => {
                    println!("Too big!");
                    check_guesses(total_guesses, secret_number, &mut auto);
                },
                Ordering::Equal => {
                    wins += 1;
                    println!("You win!!");
                    break;
                }
            }
        }
    }
}

fn check_guesses(guesses: u32, secret_number: u32, auto: &mut bool) {
    if guesses % MAX_GUESSES == 0 {
        println!("Guess so far: {guesses}");
        loop {
            println!("Oops, seems like you've run into some trouble.");
            println!("Would you like a hint? (y/n/a)");

            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            match answer.as_str().trim() {
                "y" => {
                    hint(secret_number);
                    break;
                }
                "n" =>  {
                    println!("Fair enough! Good luck!");
                    break;
                },
                "a" => {
                    println!("Auto guessing!");
                    *auto = true;
                    break;
                }
                _ => {
                    println!("I don't know what that means...");
                    continue;
                }
            }
        }
    }
}

fn hint(secret_number: u32) {
    let mut digits = Vec::new();
    let mut num = secret_number;

    while num > 9 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits.push(num); 
    digits.reverse();

    let first_digit = digits[0];

    println!("The first digit of the secret number is: {first_digit}");
}