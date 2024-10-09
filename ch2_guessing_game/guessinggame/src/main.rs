//사용자의 입력을 받고 결과값을 표시하기 위해서는 io 입
use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
  
let secrete_number = rand::thread_rng().gen_range(1..=100);

loop{
    let mut guess = String::new();

    println!("Please Input your guess");

    io::stdin().read_line(&mut guess).expect("Error Occur");

    let guess : u32= match guess.trim().parse() {
        Ok(num) => num,
        Err(_)=>continue,
    };

    match guess.cmp(&secrete_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Equal => {
            println!("You Win!");
            break;
        },
        Ordering::Greater => println!("Too big"),
    }   
}


}
