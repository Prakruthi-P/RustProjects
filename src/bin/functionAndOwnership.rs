/*
With return statement
// Syntax1
fn function_name() -> return_type {
   //statements
   return value;
}
Shorthand syntax without return statement
//Syntax2
fn function_name() -> return_type {
   value //no semicolon means this value is returned
}
*/
fn get_pi()->f64 {
    22.0/7.0
 }
 fn mutate_no_to_zero(param_no:&mut i32){
    *param_no = 0; //de reference
 }
 fn pass_by_value(mut param_no: i32) {
    param_no = param_no*0;
    println!("param_no value is :{}",param_no);
 }
 fn display(param_name:String){
    println!("param_name value is :{}",param_name);
 }
 
 fn main(){
    println!("pi value is {}",get_pi());

    /*Pass by Value */
    let num:i32 = 5;
    pass_by_value(num);
    println!("The value of no is:{}",num);

    /*Pass by Reference */

    let mut no:i32 = 5;
    mutate_no_to_zero(&mut no);
    println!("The value of no is:{}",no);

    /*Passing string to a function */

    let name:String = String::from("TutorialsPoint");
    display(name); 
    //cannot access name after displa
    //println!("The name is as follows {}",name);
}

