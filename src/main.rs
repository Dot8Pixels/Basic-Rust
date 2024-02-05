trait Player {
    fn attack(&self);
}

struct Mage {
    name: String,
    level: u8,
}

impl Player for Mage {
    fn attack(&self) {
        println!("{} is casting a fireball!", self.name)
    }
}

struct Warrior {
    name: String,
    level: u8,
    weapon: String,
}

impl Player for Warrior {
    fn attack(&self) {
        println!("{} is swinging a sword!", self.name)
    }
}

fn execute_attack<T: Player>(player: &T) {
    player.attack();
}
fn main() {
    let mage: Mage = Mage {
        name: String::from("Gandalf"),
        level: 42,
    };

    println!("name: {}, level: {}", mage.name, mage.level);

    // mage.attack();
    execute_attack(&mage);

    let warrior: Warrior = Warrior {
        name: String::from("Conan"),
        level: 1,
        weapon: String::from("Sword"),
    };

    println!(
        "name: {}, level: {}, weapon: {}",
        warrior.name, warrior.level, warrior.weapon
    );

    // warrior.attack()
    execute_attack(&warrior);
}
