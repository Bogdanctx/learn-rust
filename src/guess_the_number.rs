use std::cmp::Ordering;
use std::io::Write;
use std::io;
use rand::Rng;

pub fn guess_the_number() {
    println!("Guess the number");

    let mut tries = 0;
    let mut low = 1;
    let mut high = 500;
    let secret_number = rand::thread_rng().gen_range(low..=high);

    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();

        let mut answer = String::new();

        io::stdin()
                        .read_line(&mut answer)
                        .expect("Failed to read line");

        let answer: i32 = answer.trim().parse().expect("Please type a number!");

        match answer.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Tries: {tries}");
                break;
            }
        }

        tries += 1;
    }
}