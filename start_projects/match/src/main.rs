fn main() {
	let x = 5;
	let number:String = match x{
		1 => "new".to_owned(),
		2 => "3".to_owned(),
		_ => "Some thing not 1 or 2".to_owned(),
	};
	println!("{}", number);


	// match for enum 
	enum Message{
		Quit,
		ChangeColor(i32,i32),
		Move{x:i32, y:i32},
		Write(String),
	}
	fn quit(){
		println!("I quit");
	}
	fn change_color(a:i32, b:i32){
		println!("change color: {}, {}", a,b);
	}
	fn move_cursor(a:i32,b:i32){
		println!("move to: {}, {}", a,b);

	}
	fn write(content:String){
		println!("write as:{}", content);
	}

	let test_quit = Message::Quit;
	let test_change_color = Message::ChangeColor(1,2);
	let test_move = Message::Move{x:3,y:4};
	let test_write = Message::Write("a string test".to_owned());
	
	fn process_message(meg:Message){
		match meg{
			// Message::Quit => quit(),
			Message::ChangeColor(a,b)=>change_color(a,b),
			Message::Move{x:a, y:b}=>move_cursor(a,b),
			Message::Write(s) =>write(s),
			_ => quit(),  // contain all remaining condition, since it already
							// iterate all Message, not always neeeded here
			// _ => quit(),
		}
	}
	process_message(test_quit);
	process_message(test_write);
	process_message(test_move);
	process_message(test_change_color);

	// multiple matching
	let multiple_test = 1;
	match multiple_test{
		1|2 => println!("match 1 or 2, truth: {}",multiple_test),
		_ => println!("Others"),

	}



	// deconstructing of object
	struct Point{
		x: i32,
		y: i32,
	}

	let origin = Point{x:7, y:8};
	match origin{
		Point{y,..} =>println!("y of Point is {}",y),
	}

	match origin{
		Point{x,y} =>println!("x:{}, y:{}", x,y),
	}

	match origin{
		Point{x:x1, y:y1}=>println!("x: {}, y: {}", x1,y1),
	}

	enum OptionalTuple {
	    Value(i32, i32, i32),
	    Missing,
	}

	let x = OptionalTuple::Value(5, -2, 3);

	match x {
	    OptionalTuple::Value(..) => println!("Got a tuple!"),
	    OptionalTuple::Missing => println!("No such luck."),
	}

	let x = 5;
	match x{
		ref r => println!("get the reference to {}", r),
	}

	match x{
		1...10 => println!("get inside [1,10]"),
		_ => println!("Something else"),
	}

	struct Person{
		name: Option<String>,
	}
	let name = "steve".to_string();
	let mut x:Option<Person> = Some(Person{name:Some(name)});
	match x{
		Some(Person{name: ref a @ Some(_),..})=>println!("{:?}", a),
		_=>{},
	}

	enum OptionalInt {
	    Value(i32),
	    Missing,
	}

	let x = OptionalInt::Value(10);

	match x {
	    OptionalInt::Value(a @ _) if  a > 5 => println!("Got an int bigger than five!,{}",a),
	    OptionalInt::Value(..) => println!("Got an int!"),
	    OptionalInt::Missing => println!("No such luck."),
	}


}


