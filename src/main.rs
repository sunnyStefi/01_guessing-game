use std::io;
use std::cmp::Ordering; //type Ordering is enum (as Result) with variants Less, Greater, Equal
use rand::Rng;

/**
 * @dev Sections:
 * 1. Input from keyboad
 * 2. Generate random number
 * 3. Compare
*/
fn main(){
    //todo 
    //1 add list of words
    //2 modify the flow prompt
    let magic_number= rand::thread_rng().gen_range(1..=10); //Random index array

    println!("-------------------------------------------------------");
    println!("CIAO FERRANTE!");
    println!("I'm RUST.");
    println!("Insert a number from 1 to 10, let's see if you can guess it!");
    println!("-------------------------------------------------------");

    loop {
       
        let mut guess = String::new();
        
        io::stdin() // or std::io::stdin
        .read_line(&mut guess)
        .expect("Failed to read line"); //error handling
        
        println!("FERRANTE dice: {guess}"); //placeholder
        //shadowing used for conversion
        let guess : u32 = match guess.trim().parse(){ //expect will crash the program
            Ok(num) => num,
            Err(_) => {
                println!("Type a NUMBEEER");
                continue;
            }
        };
        match guess.cmp(&magic_number) {
            Ordering::Less => println!("RUST dice: NO BIGGER"), //arm 1
            Ordering::Greater => println!("RUST dice: no smaller"), //arm 2
            Ordering::Equal => {
                println!("SI HAI VINTO!"); 
                println!("The correct number is {magic_number}");
                println!("-------------------------------------------------------");
                break;
            },
        }
    }
}
