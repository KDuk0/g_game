extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {

          println!("Please enter your hunch.");

      let mut hunch = String::new();

       io::stdin().read_line (&mut hunch)
       .ok()
       .expect("Failed to read line");

     let hunch:u32 = match hunch.trim().parse(){
        Ok(num) => num,
        Err(_) => continue, };

       println!("Your hunch was: {}", hunch);

       match hunch.cmp(&secret_number) {
        Ordering::Less => println!("Very small!"),
        Ordering::Greater => println!("Very big!"),
        Ordering::Equal => { println!("You won!");
        break;
       }
       }
    }
}
