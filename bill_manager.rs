// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;

enum MainMenu{
    AddNewBill,
    ViewExistingBills,
    RemoveBill,
    UpdateBill,
}

impl MainMenu{
    fn from_str(s:&str) -> Option<MainMenu> {
        match s {
            "1" => Some(Self::AddNewBill),
            "2" => Some(Self::ViewExistingBills),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None
        }
    }
    fn show() {
        println!("");
        println!("========Main Menu===========");
        println!("1. Add New Bill");
        println!("2. View Existing Bills");
        println!("3. Remove Existing Bill");
        println!("4. Update Existing Bill");
        println!("============================");
        println!("Enter your choice ");
    }
}

#[derive(Debug,Clone)]
pub struct Bill{
    name: String,
    amount: f64,
}

pub struct Bills{
    inner: HashMap<String,Bill>
}

impl Bills{
    fn new() -> Self {
        Self {
            inner : HashMap::new()
        }
    }

    fn add(&mut self,bill:Bill) {
        self.inner.insert(bill.name.to_string(),bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self,name:&str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update_bill(&mut self,name:&str,amount:f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            },
            None => false
        }
    }
}


fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again ");
    }
    let input = buffer.trim().to_owned();
    if &input == ""{
        None
    }else{
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount : ");
    loop{
        let input = match get_input(){
            Some(input) => input,
            None => return None,
        };
        let parsed_input:Result<f64,_> = input.parse();
        match parsed_input {
            Ok(input) => return Some(input),
            Err(_) => println!("Please enter a valid number "),
        };
    }
}

mod menu{
    use crate::{Bill,Bills,get_bill_amount,get_input};

    pub fn add_bill(bills :&mut Bills) {
        println!("Please enter bill name : ");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Please enter bill amount : ");
        let amount = match get_bill_amount(){
            Some(input) => input,
            None => return,
        };
        let bill = Bill { name,amount };
        bills.add(bill);
        println!("Bill Added");
    }

    pub fn view_bills(bills : &Bills){
        for bill in bills.get_all() {
            println!("{:?}",bill);
        }
    }

    pub fn remove_bill(bills:&mut Bills){
        for bill in bills.get_all() {
            println!("{:?}",bill);
        }
        println!("Enter bill name to remove");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        if bills.remove(&name) {
            println!("Bill removed successfully");
        } else {
            println!("Bill does not exist");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        view_bills(&bills);
        println!("Enter bill name");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter bill amount");
        let amount = match get_bill_amount() {
            Some(input) => input,
            None => return,
        };
        if bills.update_bill(&name,amount) {
            println!("Bill updated successfully");
        } else{
            println!("Bill does not exist");
        }
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(&input) {
            Some(MainMenu::AddNewBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewExistingBills) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => println!("Please enter valid input")
        }
    }
}

fn main() {
    run_program();
}