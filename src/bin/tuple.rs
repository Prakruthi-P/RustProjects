/*
Tuple is a compound data type. A scalar type can store only one type of data. 
For example, an i32 variable can store only a single integer value. 
In compound types, we can store more than one value at a time and it can be of different types.
Tuples have a fixed length - once declared they cannot grow or shrink in size. The tuple index starts from 0.

//Syntax1
let tuple_name:(data_type1,data_type2,data_type3) = (value1,value2,value3);

//Syntax2
let tuple_name = (value1,value2,value3);
*/
fn print(x:(i32,bool,f64)){
    println!("Inside print method");
    println!("{:?}",x);
 }
 fn print_destructor(x:(i32,bool,f64)){
    println!("Inside print method");
    let (age,is_male,cgpa) = x; //assigns a tuple to  distinct variables
    println!("Age is {} , isMale? {},cgpa is  {}",age,is_male,cgpa);
 }
fn main() {
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    println!("{:?}",tuple);
    println!("integer is :{:?}",tuple.0);
    println!("float is :{:?}",tuple.1);
    println!("unsigned integer is :{:?}",tuple.2);
    let b:(i32,bool,f64) = (110,true,10.9);
    print(b);

    /*
    Destructing assignment is a feature of rust wherein we unpack the values of a tuple. 
    This is achieved by assigning a tuple to distinct variables.
     */
    let b:(i32,bool,f64) = (30,true,7.9);
    print_destructor(b);
 }