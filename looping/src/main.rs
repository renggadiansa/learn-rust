fn main() {

    //loop biasa
    // let mut counter = 0; //nilai awal sebelum looping
   
    // let result = loop {
    //     counter += 1; //saat looping result akan bertambah 1

    //     if counter == 10 { //saat looping mencapai 10 maka akan berhenti
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    //loop dengan beberapa loop
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count is {count}");
    //     let mut reamining = 10;
    
    //     loop {
    //         println!("reamining is {reamining}");
    //         if reamining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         reamining -= 1;
    //     }
    //     count += 1;
    // }

    // println!("End count is {count}");


    //loop dengan while
    // let mut number = 3;
    // while number != 0 {
    //     println!("number {number}");

    //     number -= 1;
    // }
    // println!("number {number}")

    //while dengan array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0; //karena index mulai dari 0

    while index < 5 { //panjang array a (array kelima tetap ditampilkan)
        println!("the value is {}", a[index]);

        index += 1;
    }
}
