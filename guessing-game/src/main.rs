use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess Num");

    let secret_number = rand::thread_rng().gen_range(1,101); // i32 u32 i64

    println!("Secret Number is {} ", secret_number);


    loop {
        println!("guess a num");

        // let mut foo =1;
        // let bar = foo; //immutable

        // foo = 2;

        //changeable variable use mut
        //create a empty string
        let mut guess = String::new(); 

        // we need use tis reference so add &
        io::stdin().read_line(&mut guess).expect("failed to read");
        // io :: Result ok, Err
        println!("Your num is : {}",guess);

        // it will match the result to run function
        // shadow always used to convert type
        let guess: u32 = match guess.trim().parse(){
            //num is a value from parse and we return it
            Ok(num) => num,
            // _ means no need err info
            Err(_) => continue,
        };

        // this function will convert secret_number from i32 to u32
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
    
}
