use std::io;

fn concat_string(param1: String, param2: String) {
    let result =format!("{} {}", param1, param2);
    println!("{result}");
}

fn main() {
    let mut input1 = String::new();

    let mut input2 = String::new();

    println!("=====Input Param1=====");

    io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");

    println!("=====Input Param2=====");        

    io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");

    let input1_slice = String::from(input1.strip_suffix("\r\n").unwrap());
    let input2_slice = String::from(input2.strip_suffix("\r\n").unwrap());

    concat_string(input1_slice, input2_slice);
}
