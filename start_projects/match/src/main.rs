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
			Message::Quit => quit(),
			Message::ChangeColor(a,b)=>change_color(a,b),
			Message::Move{x:a, y:b}=>move_cursor(a,b),
			Message::Write(s) =>write(s),
		}
	}
	process_message(test_quit);
	process_message(test_write);
	process_message(test_move);
	process_message(test_change_color);
}
