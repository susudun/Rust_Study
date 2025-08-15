

// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.
use std::collections::HashMap;
use std::io;

#[derive(Debug,Clone)]
struct BillOptions {
    name: String,
    amount: f32,
}

impl BillOptions {
    fn edit_bill(&mut self,input_name: String, intput_amount: f32){
    self.name = input_name;
    self.amount = intput_amount;
    }
}

fn print_beginning() {
    println!("Welcome to the Interactive Bill Manager!");
    println!("Please choose an option:");
    println!("1. Add a bill");
    println!("2. View bills");
    println!("3. Remove a bill");
    println!("4. Edit a bill");
    println!("5. Exit");
}

fn get_user_mune_number_input() -> i8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<i8>() {
        Ok(number) => return number,
        Err(e)=> println!("Error parsing input: {}", e),
    }
    return 0
}

fn set_menu_choice(number : i8, &mut bill:BillOptions) {
    match number {
        1 => {
            let mut name = String::new();
            let mut amount = String::new();
            println!("Enter billname:");
            io::stdin().read_line(&mut name).expect("Failed to read line");
            println!("Enter amount owed:");
            io::stdin().read_line(&mut amount).expect("Failed to read line");
            match amount.trim().parse::<f32>(){
                Ok (amount_value) => {
                    if ()
                }
            }
        }
    }
}

fn main() {
    let mut bill_map : HashMap<String,f32> = HashMap::new();
    let mut bill_options: BillOptions = BillOptions {
        name: "OldBill".to_string(),
        amount: 0.0,
    };
    let mut menu_choice: i8 = 0;
    bill_map.insert(bill_options.name,bill_options.amount);
    loop {
        print_beginning();
        menu_choice = get_user_mune_number_input();
        break;
    }
}