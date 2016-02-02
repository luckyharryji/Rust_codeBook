// struct Circle{
// 	x: f64,
// 	y: f64,
// 	radius:f64,
// }

// impl Circle{
// 	fn area(&self) ->f64{
// 		std::f64::consts::PI*(self.radius*self.radius)
// 	}

// 	fn grow(&self, increment:f64)->Circle{
// 		Circle{x:self.x, y:self.y,radius:self.radius+increment}
// 	}
// }


// fn main() {
//     // println!("Hello, world!");
//     let c = Circle{x:0.0, y:0.0, radius:2.0};
//     println!("{}", c.area());

//     let increase_c = c.grow(2.0).area();
//     println!("{}", increase_c);
// }


struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}

fn main() {
    let c = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}