pub trait Area{
    fn area(&self) -> f64;
}
struct RoundShape {
    redius:f64,
}
impl Area for RoundShape{
    fn area(&self) -> f64{
        3.14*self.redius*self.redius
    }
}
struct Tiangle {
    base:f64,
    height:f64,
}
impl Area for Tiangle{
    fn area(&self) -> f64{
        self.base*self.height/2.0
    }
}
struct Square {
    side_length:f64,
}
impl Area for Square{
    fn area(&self) -> f64{
        self.side_length*self.side_length
    }
}
fn calculated_area <T:Area> (r:T) -> f64{
    r.area()
}

fn main() {
    let a1 = RoundShape{
        redius:5.0,
    };
    let a2 = Tiangle{
        base:10.0,
        height:5.0,
    };
    let a3 = Square {
        side_length:6.0,
    };
    println!("a1 is {:?}",calculated_area(a1));
    println!("a1 is {:?}",calculated_area(a2));
    println!("a1 is {:?}",calculated_area(a3));
}
