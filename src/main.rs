use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
println!("I have guessed a number!");
    let st = rand::thread_rng().gen_range(1, 101);



    loop{
print!("Please, specify your guess --> ");
         io::stdout().flush().unwrap();

         let mut inp = String::new();

         io::stdin().read_line(&mut inp)
             .expect("Failed to read line");

print!("You guessed: {}", inp);
         io::stdout().flush().unwrap();


         let guess: u32 = match inp.trim().parse() {
             Ok(num) => num,
             Err(_) => {
                println!("Please, input the number!\n");
                continue;
             },
         };

         match guess.cmp(&st){
             Ordering::Less => println!("Your guess is too low\n"),
             Ordering::Greater => println!("Your guess is too high\n"),
             Ordering::Equal => {
                 println!("Your guessed it!");
                 break;
             }
         }
    }
}
