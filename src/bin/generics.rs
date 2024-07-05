/*
Generics are a facility to write code for multiple contexts with different types. 
In Rust, generics refer to the parameterization of data types and traits. 
Generics allows to write more concise and clean code by reducing code duplication and providing type-safety. 
The concept of Generics can be applied to methods, functions, structures, enumerations, collections and traits.
The <T> syntax known as the type parameter, is used to declare a generic construct. 
T represents any data-type.
*/

use std::fmt::Display;

struct Data<T> {
    value:T,
 }

 //generic functions
 fn print_pro<T:Display>(t:T){
    println!("Inside print_pro generic function:");
    println!("{}",t);
 }
 fn main() {
    //generic type of i32
    let t:Data<i32> = Data{value:350};
    println!("value is :{} ",t.value);
    //generic type of String
    let t2:Data<String> = Data{value:"Tom".to_string()};
    println!("value is :{} ",t2.value);

    //generic functions
    print_pro(10 as u8);
    print_pro(20 as u16);
    print_pro("Hello TutorialsPoint");
 }