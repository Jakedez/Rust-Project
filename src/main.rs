struct Hey{
    w: String,
    x: u16,
    y: char,
    z: bool
}

fn main() {
    let mut z: Hey;
    z = Hey {
        x: 42,
        z: true,
        y: 'c',
        w: "Bro!!!".to_string()
    };

    z.w = "AAAAAAAAAAHHHHHHHHHHH!!!!!!!!!!!!!".to_string();
    z.x = z.x + 1;
    println!("Hello, World! \n\nThe Number is: {}\n\nThe Boolean Expression is: {}\n", z.x, z.z);
    println!("The String is: {}\n\nThe Character is: {}\n", z.w, z.y);
}
