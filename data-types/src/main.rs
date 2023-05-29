use std::io;

fn main() {
    let a = [0, 1, 2, 3, 4, 5]; //index array mulai dari 0
    println!("please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    
        let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];

    println!("the value of the element at index {} is: {}", index, element);
}
