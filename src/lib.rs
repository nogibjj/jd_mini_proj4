/*A library that returns back a random movie name froom the list of the world top 10 best movies */

use rand::Rng;

//create an const array of the top 10 best movies around the world
pub const SWIMMERS: [&str; 5] = [
    "Missy Franklin",
    "Michael Phelps",
    "Ryan Lochte",
    "Katie Ledecky",
    "Simone Manuel",
];

//create a function that returns a random movie in the list above
pub fn random_swimmer() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..SWIMMERS.len());
    SWIMMERS[random_index]
}
