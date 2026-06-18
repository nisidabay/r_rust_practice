// Exercise: TrafficLight enum with a duration method.
// Each light variant has a different duration in seconds.

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 25,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    assert_eq!(red.duration(), 30);
    assert_eq!(yellow.duration(), 5);
    assert_eq!(green.duration(), 25);

    println!("All traffic light tests passed!");
}
