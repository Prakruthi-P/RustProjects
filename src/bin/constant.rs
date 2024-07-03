/*
Constants represent values that cannot be changed. 
If you declare a constant then there is no way its value changes. 
The keyword for using constants is const. Constants must be explicitly typed. 
Following is the syntax to declare a constant.

const VARIABLE_NAME:dataType = value;

The naming convention for Constants are similar to that of variables. 
All characters in a constant name are usually in uppercase. 
Unlike declaring variables, the let keyword is not used to declare a constant.

Constants v/s Variables:

=>Constants are declared using the const keyword while variables are declared using the let keyword.

=>A variable declaration can optionally have a data type whereas a constant declaration must specify the data type. 
This means const USER_LIMIT=100 will result in an error.

=>A variable declared using the let keyword is by default immutable. 
However, you have an option to mutate it using the mut keyword. 
Constants are immutable.

=>Constants can be set only to a constant expression and not to the result of a function call 
or any other value that will be computed at runtime.

=>Constants can be declared in any scope, including the global scope, 
which makes them useful for values that many parts of the code need to know about.
 */
fn main(){
   const USER_LIMIT:i32 = 100;    // Declare a integer constant
   const PI:f32 = 3.14;           //Declare a float constant

   println!("user limit is {}",USER_LIMIT);  //Display value of the constant
   println!("pi value is {}",PI);   

   /*
   Shadowing of Variables and Constants
   Rust allows programmers to declare variables with the same name. 
   In such a case, the new variable overrides the previous variable.

   Rust supports variables with different data types while shadowing.
   */
  let salary = 100.00;
  let salary = 1.50 ; 
  // reads first salary
  println!("The value of salary is :{}",salary);

  let uname = "Rust Programming";
  let uname = uname.len();
  println!("name changed to integer : {}",uname);

  const NAME:&str = "Mohtashim";
  //const NAME:usize = NAME.len(); 
   //Error : `NAME` already defined
  println!("name changed to integer : {}",NAME);
  
}