use std::io;
use rand::Rng;

fn main(){
     let mut first= String::new();
     let mut opretor = String::new();
     let mut second = String::new();
    println!("Enter first number :");
   io::stdin()
          .read_line(&mut first)
          .expect("Failed to read line");
    println!("Enter opretor :");
    io::stdin()
         .read_line(&mut opretor)
         .expect("Failed to read line");
    println!("Enter second number :");
  io::stdin()
       .read_line(&mut second)
       .expect("Failed to read line");

   let first_number: i32 = first.trim().parse().expect("Please enter a number");
   let second_number :i32 = second.trim().parse().expect("Please enter a number");
   calculetor(first_number,opretor,second_number);
 random_number();
   
 
}

 fn calculetor(first_number:i32,opretor:String,second_number:i32){
    let mut result = 0;
     match opretor.trim() {
        "+" => result = first_number + second_number,
        "-" => result = first_number - second_number,
        "*" => result = first_number * second_number,
        "/" => result = first_number / second_number,
        _ => println!("Invalid opretor"),

       
     }
     println!("Result is : {}",result);
  

 }

fn random_number() {
     
     let arr = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y', 'z'];
  println!("Enter alphabet letter :");
 let mut letter = String::new();
   io::stdin()
       .read_line(&mut letter)
       .expect("Failed to read line");
     let num = rand::thread_rng().gen_range(1..=arr.len()-1);
     let computer = arr[num];
   if letter.trim() == computer.to_string(){
       println!("You win {}", letter );
   }else{
       println!("You lose {}",computer.to_string() );
   }
    
}





