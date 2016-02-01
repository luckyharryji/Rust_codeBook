
struct Point {
	x: i32,
	y: i32,
}


struct PointRef<'a>{
	x: &'a mut i32,
	y: &'a mut i32,
}

struct Point3D{
	x: i32,
	y: i32,
	z: i32,
}
fn main() {
    let mut point = Point{x:3, y:5};
    point.x = 5;

    {
    	let r = PointRef{x:&mut point.x, y:&mut point.y};

    	*r.x = 10;
    	*r.y = 11;
    	println!("{}, {}", r.x, r.y);

    }

    let point = point;
    println!("{}, {}", point.x, point.y);


    // copy value from other object
    let mut point = Point3D{x:1, y:2, z:3};
    point = Point3D{z:4,..point};
    println!("{}, {}, {}", point.x, point.y,point.z);
    // can not copy from other type
    // let old_point = Point{x:1, y:2};
    // let new_point = Point3D{z:6, ..old_point};


	struct Inches(i32);

	let length = Inches(10);

	let Inches(integer_length) = length;
	println!("length is {} inches", integer_length);


}
