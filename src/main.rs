#[derive(Debug)]
struct Monster {
    name: String,
    health: u8,
    level: u8,
    attack: u8,
}
impl Monster {
    fn spawn(name: String, attack: u8) -> Self {
        Monster {
            name,
            health: 100,
            level: 1,
            attack,
        }
    }

    fn attack(&self) {
        println!("monster: {}, attack: {}", self.name, self.attack)
    }

    fn reduce_health(&mut self, damage: u8) {
        self.health -= damage;
    }

    fn level_up(&mut self) {
        self.level += 1;
    }
}
fn main() {
    let mut slime: Monster = Monster::spawn("Slime".into(), 10);

    slime.attack();

    slime.reduce_health(10);

    slime.level_up();

    println!(
        "name: {}, attack: {}, health: {}, level: {}",
        slime.name, slime.attack, slime.health, slime.level
    )
}
