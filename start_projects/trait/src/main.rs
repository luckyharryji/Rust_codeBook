
// code fun with trait

trait HasArea{
	fn area(&self) ->f64;
}

struct Circle {
    x: f64,
    y: f64,
}

impl HasArea for Circle{
	fn area(&self)->f64{
		std::f64::consts::PI*(self.x*self.y)
	}
}

fn print_area<T:HasArea>(shape:T){
	println!("area is : {}", shape.area());
}



trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

// can be called with T == i32
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

// can be called with T == i64
fn inverse<T>() -> T
        // this is using ConvertTo as if it were "ConvertTo<i64>"
        where i32: ConvertTo<T> {
    42.convert()
}

fn main() {
	let test_circle = Circle{
		x:1.0f64,
		y:1.0f64,
	};
	print_area(test_circle);
}
