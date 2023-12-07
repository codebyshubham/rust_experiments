#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn give_away(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        match user_preference {
            Some(color) => {
                self.remove_shirt(color);
                color
            },
            None => {
                let color = self.most_stocked();
                self.remove_shirt(color);
                color
            }
        }
    }

    fn remove_shirt(&mut self, color: ShirtColor) {
        let pos = &self.shirts.iter().position(|c| *c == color).unwrap();
        let _ = &self.shirts.remove(*pos);
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut count_red = 0;
        let mut count_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => count_red += 1,
                ShirtColor::Blue => count_blue += 1
            }
        }

        if count_red > count_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let mut store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_one_preference = Some(ShirtColor::Blue);

    let user_one_tshirt = store.give_away(user_one_preference);

    let user_two_preference = None;

    let user_two_tshirt = store.give_away(user_two_preference);

    println!("User one: {:?}", user_one_tshirt);
    println!("User two: {:?}", user_two_tshirt);
}