/*
In Rust, errors can be classified into two major categories as shown in the table below.

No	Name & Description	                                Usage
1	Recoverable=Errors which can be handled             Result enum
2	UnRecoverable=Errors which cannot be handled        panic macro

A recoverable error is an error that can be corrected. 
A program can retry the failed operation or specify an alternate course of action when it encounters a recoverable error. 
Recoverable errors do not cause a program to fail abruptly. 
An example of a recoverable error is File Not Found error.

Unrecoverable errors cause a program to fail abruptly. 
A program cannot revert to its normal state if an unrecoverable error occurs. 
It cannot retry the failed operation or undo the error. 
An example of an unrecoverable error is trying to access a location beyond the end of an array.

Unlike other programming languages, Rust does not have exceptions. 
It returns an enum Result<T, E> for recoverable errors, while it calls the panic macro if the program encounters an unrecoverable error. 
The panic macro causes the program to exit abruptly.
*/
fn main(){
//Panic Macro and Unrecoverable Errors
/*
anic! macro allows a program to terminate immediately and provide feedback to the caller of the program. 
It should be used when a program reaches an unrecoverable state.
*/
let no = 1; 
   //try with odd and even
   if no%2 == 0 {
      println!("Thank you , number is even");
   } else {
      panic!("NOT_AN_EVEN"); 
   }
   println!("End of main");

let a = [10,20,30];
   a[10];
panic!("Hello");
//println!("End of main"); //unreachable statement
}