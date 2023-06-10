use rand::Rng;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigh
    }
}

fn main() {
    let width = rand::thread_rng().gen_range(1..=100);
    let heigh = rand::thread_rng().gen_range(10..=50);
    let rect = Rectangle {
        width: width,
        heigh: heigh,
    };
    dbg!(rect.area());
}
