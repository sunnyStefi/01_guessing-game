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
    give_me_tons_of_cars();
    //guessing_number();
}


fn give_me_tons_of_cars(){
    let mut number_of_cars = String::new();
    let mut type_of_car = String::new();
    let mut result = String::new();
    let mut parsed_number_of_cars : u32;
    
    loop{
        println!("How many cars do you want to see?");
        io::stdin().read_line(&mut number_of_cars).expect("Fail to read line");  
        parsed_number_of_cars = match number_of_cars.trim().parse(){ //expect will crash the program
                Ok(num) => {num},
                Err(_) => {
                    println!("{}", number_of_cars);
                    println!("Type a number");
                    number_of_cars = String::new();
                    continue;
                }
        };
        break;
        
    }
    println!("Do you want to see POLICE cars or a FORMULA 1 cars?");
    io::stdin().read_line(&mut type_of_car).expect("Fail to read line");
    let type_of_car = type_of_car.to_lowercase();
    
    show_car(parsed_number_of_cars, type_of_car);
}


fn show_car(x : u32, car_type : String) {
    let police = "🚓";
    let formula1 ="🏎️";
    let mut chosen_car = "💩";
    if car_type.trim() == "police" 
        { chosen_car = police;} 
    if car_type.trim() == "formula 1" 
        {chosen_car = formula1;} 

    for repetition in 0..x {
            print!("{} {}  ",repetition+1, chosen_car);
    }
}


fn guessing_number(){
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
                println!("Type a NUMBER");
                continue;
            }
        };
        match guess.cmp(&magic_number) {
            Ordering::Less => println!("RUST dice: NO BIGGER"), //arm 1
            Ordering::Greater => println!("RUST dice: no smaller"), //arm 2
            Ordering::Equal => {
                println!("SI H~AI VINTO!"); 
                println!("The correct number is {magic_number}");
                println!("-------------------------------------------------------");
                break;
            },
        }
    }
}
