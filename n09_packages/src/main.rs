/* Use */
use crate::garden::vegetables::Asparagus;
// use crate::garden::instruments::Shovel; // Struct {}
// use crate::garden::instruments; // instruments::Struct {}
use crate::garden::instruments as i; // ins::Struct {}

/* Mod */
pub mod garden;

/* Entry point */
fn main() {
    let plant = Asparagus {};
    
    let shovel = i::Shovel {};
    let scissors = i::Scissors {};
    
    println!("Plant: {:?}\nInstruments: {:?}, {:?}", plant, shovel, scissors);
}
