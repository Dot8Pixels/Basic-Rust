use std::result::Result;

fn get_dog_price(breed: &String) -> Result<u32, String> {
    match breed.as_str() {
        "Chihuahua" => Ok(10000),
        "Labrador" => Ok(20000),
        "GermanShepherd" => Ok(30000),
        _ => Err(String::from("Breed not found")),
    }
}
fn main() {
    let dog: String = String::from("Labrador");

    let price: u32 = match get_dog_price(&dog) {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            0
        }
    };

    println!("{}", price);
}
