fn main() {
	let v = vec![1,2,3,4];
	let v2 = &v; // only with reference, v2 only borrow ownership, v still bind to resource

	// println!("v[0] :{}", v[0]);  error: only on binding to the given resources vec![1,2,3,4], now the binding is on v2
	println!("v2[0] : {}, v1[0]:{}", v2[0],v[0]);

	let new_v = vec![1,2,3];
	take_ownership(new_v);

	// println!("new_v[0] : {}", new_v[0]); Error: ownership has been bing to function

	// copy trait
	// primitive type implement the copy trait, instead a pointer to a data reseourse in stack, it copy the data
	let x = 5;
	let y = double(x);
	println!("access y: {}, x: {}", y,x);


	// complicate way to give back ownership
	let v1 = vec![1,2,3];
	let (v1,result) = give_back_ownership(v1);
	println!("v1 still bing to resource, v1[0]: {}, result :{}", v1[0],result);



	// scope of mutable borrow has to be narrow than the resoure it link to
	let mut x = 5;

	{
	    let y = &mut x; // -+ &mut borrow starts here
	    *y += 1;        //  |
	}                   // -+ ... and ends here

	println!("{}", x);  // <- try to borrow x here


	// resource live longer than reference
	// let  y:&i32;
	// {
	// 	let x = 5;
	// 	y = &x;
	// 	println!("x: {}, y:{}",x,y);
	// }

	let y =&5;
	let life_time_function = Foo{x:y};

}

static Foo:i32 = 5;


// control of life time for function and struct
pub struct Foo<'a> {
    x: &'a i32,
}


impl<'a> Foo<'a>{
	fn x(&self) -> &'a i32{self.x}
}


fn double(x:i32) ->i32{
	x*2
}


fn take_ownership(v:Vec<i32>){
	println!("length: {}", v.len());
}

fn give_back_ownership(v:Vec<i32>) ->(Vec<i32>,i32){
	(v,32)
}