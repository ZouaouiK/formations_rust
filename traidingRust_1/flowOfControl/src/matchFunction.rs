pub fn match_function(){
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("number is {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
      
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
       
}
}

pub fn match_function1(){
    let trainee = Trainee::Bacem(9u32); //refernce non type copye
    let trainee1 = Trainee::Khouka(true);
    let trainee2 = Trainee::Amal("Amal2".to_string());

    let t1 = &trainee2;
    use Trainee::*;
    let a = match t1 {
          Bacem(9)=> println!("second one "),
        Bacem(x)=> println!("{}",x),
        Amal(x)=>println!("{}",x),
        _ => println!(" i dont know "), 
        /* Bacem(9) => 9u32,
        Bacem(x) => *x,
        Amal(x) => x.len() as u32,
        _ => {
            println!(" i dont know ");
            10u32
        } */
    };
    //println!("a = {}", a);
}
pub enum Trainee {
    Bacem(u32),
    Khouka(bool),
    Amal(String), //traja3 string
}