/*
Readers are components that your program can read bytes from. Examples include reading input from the keyboard, files, etc. 
The read_line() method of this trait can be used to read data, one line at a time, from a file or standard input stream.

Writers are components that your program can write bytes to. Examples include printing values to the console, writing to files, etc. 
The write() method of this trait can be used to write data to a file or standard output stream.
*/
use std::io::stdin;
use std::io::Write;
use std::io::Read;


fn main(){
    // let mut line = String::new();
    // println!("Enter your name :");
    // let b1 = std::io::stdin().read_line(&mut line).unwrap();
    // println!("Hello , {}", line);
    // println!("no of bytes read , {}", b1);
    // let mut occupation=String::new();
    // println!("Enter your occupation");
    // stdin().read_line(&mut occupation).expect("Failed to read line");
    // println!("The occupation is :{}",occupation);

    // //write....
    // let b1 = std::io::stdout().write("Coffee".as_bytes()).unwrap();
    // let b2 = std::io::stdout().write(String::from("Beans").as_bytes()).unwrap();
    // std::io::stdout().write(format!("\nbytes written {}",(b1+b2)).as_bytes()).unwrap();

    // //file input 
    // let cmd_line = std::env::args();
    // println!("No of elements in arguments is :{}",cmd_line.len()); 
    // //print total number of values passed
    // for arg in cmd_line {
    //    println!("[{}]",arg); //print all values passed as commandline arguments
    // }

    // //write to file
    // let mut file = std::fs::File::create("data.txt").expect("create failed");
    // file.write_all("Hello World".as_bytes()).expect("write failed");
    // file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
    // println!("data written to file" );

    //Read from a File
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
 
 }
 