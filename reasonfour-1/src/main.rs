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

fn main() {
    let input1 = TrafficLight::Yellow;
    let input2 = TrafficLight::Red;
    let input3 = TrafficLight::Green;
    println!("The Yellow time is {} !", input1.time());
    println!("The Red time is {} !", input2.time());
    println!("The Green time is {} !", input3.time());
}

