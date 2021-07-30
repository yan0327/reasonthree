/*实现一个信号灯trait*/
trait LightTime{
    fn time(&self) ->  u8 ; 
}

enum TrafficLight{
    Red,
    Green,
    Yellow,
}

impl LightTime for TrafficLight {
    fn time(&self) -> u8{
        let output = match self{
            TrafficLight::Red => 60,
            TrafficLight::Green => 75,
            TrafficLight::Yellow => 3,
        };
        output
    }
}
/******************/
/*实现u32类型的整数求和，并返回Option*/
fn Sum (arr:&[u32]) -> Option<u32>{
    let mut sum:u32 = 0;
    let MAX:u32 = 2147483648;
    let mut flag =false;
    for element in arr.iter(){
        if MAX - sum < *element{
            flag = true;
            break;
        }
        sum = sum + element;
        
    }
    if flag == true{
        None
    }else{
        Some(sum)
    }
        
}
/******************/  
 /*实现一个打印图形面积的函数*/   
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
    SideLength:f64,
}
impl Area for Square{
    fn area(&self) -> f64{
        self.SideLength*self.SideLength
    }
}
fn CalculatedArea <T:Area> (r:T) -> f64{
    r.area()
}
/******************/  
fn main() {
    /*实现一个信号灯trait*/
    let input = TrafficLight::Yellow;
    println!("The time is {} !", input.time());
    /******************/
    /*实现u32类型的整数求和，并返回Option*/
    let a = [10,20,30,40,50,2147483648];
    let b = [10,20,30,40,50,60,70];
    let output = Sum(&a);
    println!("This sumA is {:?}", output);
    println!("This sumB is {:?}", output);
    /******************/  
    /*实现一个打印图形面积的函数*/
    let a1 = RoundShape{
        redius:5.0,
    };
    let a2 = Tiangle{
        base:10.0,
        height:5.0,
    };
    let a3 = Square {
        SideLength:6.0,
    };
    println!("a1 is {:?}",CalculatedArea(a1));
    println!("a1 is {:?}",CalculatedArea(a2));
    println!("a1 is {:?}",CalculatedArea(a3));
    /******************/  
}

