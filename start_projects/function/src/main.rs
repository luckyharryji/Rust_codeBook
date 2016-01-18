fn main() {



	let print_f:fn(i:i32) = print_number;

	let function_variable = print_f(5);

	let add_f:fn(i:i32)->i32 = add_one;
	let add_test = add_f(6);
	println!("test add function binding: add_test {}", add_test);


	print_sum_of_two_numbwe(3,5);
	println!("{}", add_one(9));
	let x:String = diverges();


	// expression and statement
	// let mut y = 5;
	//let x = y = 6;
	// println!("x: {}, y: {}", x, y); //Error: x has the empty tuple (); assigned value(5) can only have one owner
	
}

fn diverges() -> !{
	panic!("This diverge function never return , it crash the thread");
}

fn print_number(x:i32){
	println!("value of x is : {}", x);
}

fn print_sum_of_two_numbwe(x:i64, y:i64){
	println!("sum of {} and {} is : {}",x,y,x+y );
}

// expression based
fn add_one(x:i32) -> i32{
	//do anything

	let show_x = x;
	println!("x is {}", show_x);


	println!("x + 1 equals to :");

	// last line of the function returns the value
	x + 1
}