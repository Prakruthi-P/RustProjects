/*
Readers are components that your program can read bytes from. Examples include reading input from the keyboard, files, etc. 
The read_line() method of this trait can be used to read data, one line at a time, from a file or standard input stream.

Writers are components that your program can write bytes to. Examples include printing values to the console, writing to files, etc. 
The write() method of this trait can be used to write data to a file or standard output stream.

1   std::fs::File	        open()	            pub fn open<P: AsRef>(path: P) -> Result	
The open static method can be used to open a file in read-only mode.

2	std::fs::File	        create()	        pub fn create<P: AsRef>(path: P) -> Result	
Static method opens a file in write-only mode. If the file already existed, the old content is destroyed. Otherwise, a new file is created.

3	std::fs::remove_file	remove_file()	    pub fn remove_file<P: AsRef>(path: P) -> Result<()>	
Removes a file from the filesystem. There is no guarantee that the file is immediately deleted.

4	std::fs::OpenOptions	append()	        pub fn append(&mut self, append: bool) -> &mut OpenOptions	
Sets the option for the append mode of file.

5	std::io::Writes	        write_all()	        fn write_all(&mut self, buf: &[u8]) -> Result<()>	
Attempts to write an entire buffer into this write.

6	std::io::Read	        read_to_string()	fn read_to_string(&mut self, buf: &mut String) -> Result	
Reads all bytes until EOF in this source, appending them to buf.
*/
use std::io::stdin;
use std::io::Write;
use std::io::Read;
use std::fs;
use std::fs::OpenOptions;


fn main(){
    let mut line = String::new();
    println!("Enter your name :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
    println!("no of bytes read , {}", b1);
    let mut occupation=String::new();
    println!("Enter your occupation");
    stdin().read_line(&mut occupation).expect("Failed to read line");
    println!("The occupation is :{}",occupation);

    //write....
    let b1 = std::io::stdout().write("Coffee".as_bytes()).unwrap();
    let b2 = std::io::stdout().write(String::from("Beans").as_bytes()).unwrap();
    std::io::stdout().write(format!("\nbytes written {}",(b1+b2)).as_bytes()).unwrap();

    //file input 
    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len()); 
    //print total number of values passed
    for arg in cmd_line {
       println!("[{}]",arg); //print all values passed as commandline arguments
    }

    //write to file
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
    println!("data written to file" );

    //Read from a File
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    //delete a file
    fs::remove_file("data.txt").expect("could not remove file");
    println!("\nfile is removed");

    //append data to a file
    let mut file = OpenOptions::new().append(true).open("inputFile.txt").expect(
        "cannot open file");
     file.write_all("Hello World".as_bytes()).expect("write failed");
     file.write_all("\nTutorialsPoint".as_bytes()).expect("write failed");
     println!("file append success");
 
 }
 