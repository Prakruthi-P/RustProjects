/*Enum Result – <T,E> can be used to handle recoverable errors. 
It has two variants − OK and Err. T and E are generic type parameters. 
T represents the type of the value that will be returned in a success case within the OK variant, 
and E represents the type of the error that will be returned in a failure case within the Err variant. 

enum Result<T,E> {
   OK(T),
   Err(E)
}*/
use std::fs::File;

fn is_even(no:i32)->Result<bool,String> {
    if no%2==0 {
       return Ok(true);
    } else {
       return Err("NOT_AN_EVEN".to_string());
    }
 }
fn main() {
   //The program returns OK(File) if the file already exists and Err(Error) if the file is not found. 
    let f = File::open("main.jpg"); 
    println!("{:?}",f);
    //The following example handles an error returned while opening file using the match statement
   let f = File::open("main.jpg");   // main.jpg doesn't exist
   match f {
      Ok(f)=> {
         println!("file found {:?}",f);
      },
      Err(e)=> {
         println!("file not found {:?}",e);   //handled error
      }
   }
   let result = is_even(10);
   match result {
      Ok(d)=>{
         println!("Number is even {}",d);
      },
      Err(msg)=>{
         println!("Error msg is {}",msg);
      }
   }

   println!("end of main");
}