use std::cmp::Ordering;
use rand::Rng;

pub fn automated_bin_src() {
    println!("Binary search run");

    let limit = 256;
    let mut low = 1;
    let mut high = limit;
    let mut tries = 0;
    let random_number = rand::thread_rng().gen_range(low..=limit);

    loop {
        let middle = low + (high - low) / 2;

        match middle.cmp(&random_number) {
            Ordering::Less => {
                println!("{middle} too low.");
                low = middle + 1;
            },
            Ordering::Greater => {
                println!("{middle} too high.");
                high = middle - 1;
            },
            Ordering::Equal => {
                println!("{middle} found. Binary search completed in {tries} tries!");
                break;
            }
        }
        tries += 1;
    }
}