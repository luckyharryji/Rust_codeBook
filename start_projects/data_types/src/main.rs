fn main() {

	// let x = true;
	// let y:bool = false;

	// let a = [0;20];
	let a = [1,2,3,4];
	println!("length of a: {}", a.len());
	println!("second element of a: {}", a[1]);

	for x in &a{
		print!("{}", x);
	}

	let b = [0,1,2,3,4];
	let complete_b = &b[..];
	let middle_b = &b[1..4];
	println!("{}", complete_b[1]);

	for data in complete_b{  // here, complete_b is already a reference
		print!("{}", data);
	}

	// tuple
	let mut x = (1, "Hello");
	let y:(i32,&str) = (3,"World");
	x = y;

	println!("test indexing: {}", x.1);

	let (a,b,c) = (1,2,3);
	println!("{}", a);

	let x:fn(i32) -> i32 = foo;
}


fn foo(x:i32) -> i32{
	x
}