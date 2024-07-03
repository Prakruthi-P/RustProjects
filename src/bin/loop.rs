/*
For Loop
The for loop executes the code block for a specified number of times. It can be used to iterate over a fixed set of values, such as an array. The syntax of the for loop is as given below

Syntax
for temp_variable in lower_bound..upper_bound {
   //statements
}
*/
fn main(){
    for x in 1..10{ // 11 is not inclusive
        if x%5==0 {
           continue;
        }
        println!("x is {}",x);
     }
     let mut x = 0;
     while x < 10{
        x+=1;
        println!("inside loop x value is {}",x);
     }
     println!("outside loop x value is {}\n",x);

     let mut x = 0;
     loop {
        x+=1;
        println!("x={}",x);
  
        if x==15 {
           break;
        }
     }
}
