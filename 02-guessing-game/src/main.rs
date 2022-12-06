use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Try to guess a number, if you can. :p!");
    let mut guess_count:u32 = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Input a number between 1-100");
        let mut guessed_input = String::new();
        io::stdin()
            .read_line(&mut guessed_input)
            .expect("Unable to read input");
    
        let parsed_guess: i32 = match guessed_input.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("Unable to convert the data. Please try again.");
                continue;
            },
        };

        guess_count += 1;
    
        println!("You guessed {parsed_guess}. Number of guess: {guess_count}");
        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Its too small, aim larger.\n"),
            Ordering::Equal => {
                println!("Yess you got it.\n");
                break;
            },
            Ordering::Greater => println!("Too big, aim smaller.\n"),
        }    
    }
    println!("CONGRATULATIONS. You completed in {guess_count} tries.");

}
