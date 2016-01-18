fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("{}", x);

    let (a,b) = (1,2);  // initial with pattern
    println!("{} {}", a,b);

    let type_initial : i32 = 5; // initial with a type
    println(type_initial);  // calling defined function

    let mut mutable_variable:i32 = 5;  // initial multable variable
    mutable_variable = 23;
    println(mutable_variable);


    let unintialized_variable:i32;
    // println!("{}", unintialized_variable); // Error, {} need to interploate to a specific type

    let outside_block:i32 = 5;
    {
    	let inside_block:i32 = 17;
    	println!("{},{}", outside_block, inside_block);
    }
    // println!("{}, {}", outside_block, inside_block);  // Error : variable bind inside the block can not be accessed outside


    // shadow and mutable
    let shadow_variable:i32 = 8;
    {
    	println!("should be 8:  {}", shadow_variable);
    	// shadow_variable = 10;  // can not be assigned to immutable
    	let shadow_variable = 12;
    	println!("should be 12 {}", shadow_variable);
    }
    println!("should be 8: {}", shadow_variable);
    let mut shadow_variable = 32;
    println!("should be 32: {}", shadow_variable);

    shadow_variable = 41;
    println!("should be 41: {}", shadow_variable);


}



fn println(x:i32){
	println!("{}", x);
}