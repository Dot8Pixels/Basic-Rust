#[derive(Debug)]

enum PlayerClass {
    Mage(i32),
    Warrior(i32),
}

trait PlayerAction {
    fn get_weapon(&self);
}

impl PlayerAction for PlayerClass {
    fn get_weapon(&self) {
        match self {
            Self::Mage(damage) => println!("{}, damage: {}", String::from("Staff"), damage),
            Self::Warrior(damage) => println!("{}, damage: {}", String::from("Sword"), damage),
        }
    }
}
fn main() {
    let mage_class: PlayerClass = PlayerClass::Mage(200);

    println!("{:?}", mage_class);

    mage_class.get_weapon();

    let warrior_class: PlayerClass = PlayerClass::Warrior(150);

    println!("{:?}", warrior_class);

    warrior_class.get_weapon()
}
