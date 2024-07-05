/*
unwrap() and expect()
The standard library contains a couple of helper methods that both enums − Result<T,E> and Option<T> implement. 
You can use them to simplify error cases where you really do not expect things to fail. 
In case of success from a method, the "unwrap" function is used to extract the actual result.

unwrap(self): T
Expects self to be Ok/Some and returns the value contained within. 
If it is Err or None instead, it raises a panic with the contents of the error displayed.

expect(self, msg: &str): T
Behaves like unwrap, except that it outputs a custom message before panicking in addition to the contents of the error.
*/
use::std::fs::File;
fn main(){
    let result = is_even(10).unwrap();
    println!("result is {}",result);
    
    let f=File::open("input.txt").expect("File not able to open\n");
 }
 fn is_even(no:i32)->Result<bool,String> {
    if no%2==0 {
       return Ok(true);
    } else {
       return Err("NOT_AN_EVEN".to_string());
    }
 }