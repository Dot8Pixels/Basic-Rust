// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn get_x(&self) -> &T {
//         &self.x
//     }

//     fn get_y(&self) -> &T {
//         &self.y
//     }
// }
trait PlayerActions {
    fn attack(&self);
}
struct Warrior;

impl PlayerActions for Warrior {
    fn attack(&self) {
        println!("Warrior Attack");
    }
}

struct Mage;

impl PlayerActions for Mage {
    fn attack(&self) {
        println!("Mage Attack");
    }
}

fn attack<T: PlayerActions>(player_class: &T) {
    player_class.attack()
}

fn main() {
    // let p1: Point<f64> = Point { x: 1.5, y: -7.3 };
    // let p2 = Point { x: 2, y: -3 };

    // println!("A ({}, {})", p1.get_x(), p1.get_y());
    // println!("B ({}, {})", p2.get_x(), p2.get_y());

    let warrior = Warrior;
    let mage = Mage;

    attack(&warrior);
    attack(&mage)
}
