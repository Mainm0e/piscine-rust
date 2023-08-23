/* Instructions
Imagine you are working on some software to control smart lights in a house. You have access to an array of all the lights in that house.

Define the associated function new, and add it to the data structure Light. It should create a new light with the alias passed as an argument, with a brightness of 0.

Define the function change_brightness, which receives a Vec of lights, an alias and a u8value. It should find the light in the Vec by its alias, and set the value of the brightness.

Expected Functions and Structure
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
	}
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
}
 */

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
    for light in lights {
        if light.alias == alias {
            light.brightness = value;
        }
    }
}