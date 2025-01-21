use rand::Rng;
use std::io;

fn dice_roll(sides: u32) -> u32 {
    let mut rng = rand::thread_rng();
    
    return rng.gen_range(1..=sides);
}

fn main() {
    println!("Specify the number of sides");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error while reading input");
    let input = input.trim();
    
    let sides: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    println!("Rolled {}", dice_roll(sides));
}
