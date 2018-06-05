pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("I'm very nested", )
            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use a::series::of;
use a::series::of::nested_modules;
use TrafficLight::{Red, Yellow}; // Bring only Red and Yellow to scope
use TrafficLight::*; // Bring every color to scope

fn main() {
    a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();

    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;

    let _red = Red;
    let _yellow = Yellow;
    let _green = Green;
}
