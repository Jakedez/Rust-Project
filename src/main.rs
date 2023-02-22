use std::{io::{stdin,stdout,Write}};



struct Person{
    name: String,
    age: i32,
    career: String
}






fn get_int(dialog : String) -> i32{
    print!("{dialog}");
    stdout().flush().unwrap();
    let _ = stdout().flush();
    let mut input_line = String::new();
    stdin().read_line(&mut input_line).expect("Failed to read line");
    let x: i32 = input_line.trim().parse().expect("Input not an integer");
    return x;
}

fn get_string(dialog : String) -> String{
    let mut return_string:String = String::new();
    print!("{dialog}");
    stdout().flush().unwrap();
    let _ = stdout().flush();
    stdin().read_line(&mut return_string).expect("An Error Has occured.");
    if let Some('\n')=return_string.chars().next_back() {
        return_string.pop();
    }
    if let Some('\r')=return_string.chars().next_back() {
        return_string.pop();
    }

    return return_string;

}

fn get_person() -> Person{
    let person: Person = Person { name: get_string("Please type a name: ".to_string()), age: get_int("Please type an age: ".to_string()), career: get_string("Please enter your career choice: ".to_string())};
    return person; 
}

fn read_age(person : Person) -> Person{
    println!("\n\n\nAge: {}\n\n\n", person.age);
    return person;
}

fn print_menu(){
    println!("A. Change Career");
    println!("B. Change all account info");
    println!("C. Get Current Age");
    println!("D. Display all info");
    println!("E. Exit");
    return;
}

fn display_all(account : Person) -> Person{
    println!("\n\n\nName: {}", account.name);
    println!("Age: {}", account.age);
    println!("Career: {}\n", account.career);
    println!("");
    return account;
}

fn main() {
    let mut account : Person = get_person();
    println!("\n\n\n");

    let mut running : bool = true;

    let mut response : String;

    while running {
        print_menu();
        response = get_string("Please select an option: ".to_string()).to_lowercase();

        if response == "e".to_string(){
            running = false;
            println!("\n\n\n");
        }
        else if response == "d".to_string(){
            account = display_all(account);
        }
        else if response == "c".to_string(){
            account = read_age(account);
        }
        else if response == "b".to_string() {
            println!("\n\n\n");
            account = get_person();
            println!("\n\n\n");
        }
        else if response == "a".to_string(){
            account.career = get_string("\n\n\nPlease type a new career choice: ".to_string());
            println!("\n\n\n");
        }
        else{

        }
        


    }

    println!("Goodbye {}!", account.name);
    
    return;
}
