fn main() {

	let x = 5;
	if x == 5{
		println!("X is 5, enter the if condition");
	}
	else if x == 6{
		println!("X is 6");
	}
	else{
		println!("X is not 5/6, enter the else condition");
	}

	let mut y = if x == 5{x} else{x+1};

	let mut done = false;

	// while loop
	while !done{
		println!("{}", y);
		if y == 12{
			done = true;
		}
		y += 1;

	}

	// rewrite with loop
	let mut y = 10;
	println!("beging LOOP loop: ");
	loop{
		println!("{}", y);
		y += 1;
		if y == 20{
			break;
		}
	}

	// for loop
	println!("FOR loop:");
	for x in 1..11{
		println!("{}", x);
	}

	// enumerate
	for (index,data) in (5..10).enumerate(){
		println!("index:{},  data:{}",index,data );
	}

	// label for the loop 
	// control which to break/continue
	println!("Test for add label for loop");
	'outer: for x in 0..10{
		'inner: for y in 0..10{
			if x%2 == 0{
				continue 'outer;
			}
			if y%2 ==0{
				continue 'inner;
			}
			println!("x: {}, y: {}", x,y);
		}
	}
}
