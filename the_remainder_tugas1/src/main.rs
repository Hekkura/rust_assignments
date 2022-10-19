use std::io;

fn modulo(param1:i32,param2:i32) -> i32 {
    param1 % param2
}

fn display_result(result: i32){
    println!("{}",result);

}

fn main(){
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

    let input_parsed1 = input1
        .trim()
        .parse()
        .expect("Input not an integer");

    let input_parsed2 = input2
        .trim()
        .parse()
        .expect("Input not an integer");

    let result = modulo(input_parsed1,input_parsed2);
    display_result(result);
}
