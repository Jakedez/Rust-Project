use std::io::{stdin,stdout,Write};


struct Person{
    name: String,
    age: i32
}

fn get_char(dialog : String) -> char{
    let character : char;
    print!("{dialog}");
    stdout().flush().unwrap();
    let _ = stdout().flush();
    let mut input_line: String = String::new();
    return 'c';
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
    let person: Person = Person { name: get_string("Please type a name: ".to_string()), age: get_int("Please type an age: ".to_string()) };
    return person; 
}

fn main() {
    let hello: Person = get_person();
    println!("Name: {}, Age: {}", hello.name, hello.age);
    let mut running : bool = true;
    while running {
        running = false;
    }
    return;
}
