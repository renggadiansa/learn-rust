//function yang dimasukan ke variable
// fn five() -> i32 {
//     5
// }


fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    // function dimasukan kedalam variable
    // let x = five();
    // println!("The value of x is: {x}");
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(value: i32, unit_label: char) {
    println!("the value of x is {value}, {unit_label}");
}