
// use std::io::{BufRead,BufReader,Read,stdin};

use std::io::prelude::*;
use std::fs::File;
fn main(){
    foo();
    read_file("test.txt");
}

fn read_file(file_name:&str) -> std::io::Result<()>{
	let mut f = try!(File::open(file_name));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	println!("Sting in the file is : {}", s);
	Ok(())
}

fn foo() -> std::io::Result<()> {
	let mut f = try!(File::create("foo.txt"));
	try!(f.write_all(b"Hello, world!"));

	let mut f = try!(File::open("foo.txt"));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	Ok(())
}


// fn read_measurements<R: Read>(reader: R) -> Vec<f64> {
//     let mut measurements: Vec<f64> = vec![]; // Vec::new()
//     let mut lines = BufReader::new(reader).lines();

//     while let Some(Ok(line)) = lines.next() {
//         if line == "999" {break}

//         if let Ok(f) = line.parse() {
//             if f >= 0.0 {
//                 measurements.push(f);
//             }
//         }
//     }

//     for line in &measurements{
//     	println!("{}", line);
//     }

//     return measurements;
// }