fn main() {
    /*
    The name of a variable can be composed of letters, digits, and the underscore character.
    It must begin with either a letter or an underscore.
    Upper and lowercase letters are distinct because Rust is case-sensitive.
    The syntax for declaring a variable is given below.
    let variable_name = value;   --no type specified
    let variable_name:dataType = value;  ---type specified
    */
    let fees = 25_000;
    let salary:f64 = 35_000.00;
    println!("fees is {} and salary is {}\n",fees,salary);

    /*
    By default, variables are immutable − read only in Rust. 
    In other words, the variable's value cannot be changed once a value is bound to a variable name. 
    */

    let fees = 25_000;
    println!("fees is {} ",fees);
    //fees = 35_000;
    println!("fees not changed is {}\n",fees);

    /*
    Mutable
    Variables are immutable by default. 
    Prefix the variable name with mut keyword to make it mutable. 
    The value of a mutable variable can be changed.
     */

     let mut fees:i32 = 25_000;
     println!("fees is {} ",fees);
     fees = 35_000;
     println!("fees changed is {}\n",fees);
}
