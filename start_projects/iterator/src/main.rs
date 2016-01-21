fn main() {

	let great_than_forty_two = (0..100).find(|x| *x>42);

	// match great_than_forty_two{
	// 	Some(_) => println!("{}", great_than_forty_two),
	// 	None => println!("Not find"),
	// }
	for num in great_than_forty_two{
		println!("{}", num);
	}

	let sum = (1..4).fold(0, |sum, x| sum + x);

	println!("Sum: {}", sum);
	
	let values = vec![1, 2, 3, 4, 5];
	{
	    let result = match values.into_iter() {
	        mut iter => loop {
	            match iter.next() {
	                Some(x) => { println!("{}", x); },
	                None => break,
	            }
	        },
	    };
	    result
	}



}
