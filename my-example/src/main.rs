use rand::Rng;
fn main() {
    let width = rand::thread_rng().gen_range(1..=100);
    let heigh = rand::thread_rng().gen_range(100..=200);
    println!("area is {}", area(width, heigh));
    let data = (width, heigh);
    area1(data);
    println!("data area is {}", area(width, heigh));

    let rect = Rectangle {
        width: width,
        heigh: heigh,
    };
    println!("rect area is {:?}", rectangle(&rect));
    println!("rect area is {:?}", rect);
    println!("rect area is {:#?}", rect);

    let rect1 = Rectangle {
        width: dbg!(30 * width),
        heigh: heigh,
    };
    dbg!(&rect1);
}

fn area(width: u32, heigh: u32) -> u32 {
    width * heigh
}
// 使用元组的结构体
fn area1(data: (u32, u32)) -> u32 {
    data.0 * data.1
}

fn rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.heigh
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}
