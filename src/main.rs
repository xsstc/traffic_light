pub trait Timer {
    fn time(&self) -> usize;
}

enum TrafficLight {
    Green,
    Yellow,
    Red,
}

impl Timer for TrafficLight {
     fn time(&self) ->usize {
        match self {
            TrafficLight::Green => {println!("Green last 1s");1},
            TrafficLight::Yellow => {println!("Yellow last 2s");2},
            TrafficLight::Red =>{println!("Red last 3s");3},
        }
    }
}

fn main() {
    let light_1 = TrafficLight::Green;
    let light_2 = TrafficLight::Yellow;
    let light_3 = TrafficLight::Red;

    light_1.time();
    light_2.time();
    light_3.time();
}
