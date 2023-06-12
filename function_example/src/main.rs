use rand::Rng;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// imp快，表示该代码快中所有内容都是与 Rectangle 类型相关的
// 所有定义在imp中的函数（非方法，也就是第一个参数不是self） -- 关联函数
impl Rectangle {
    // 使用&self替代 Rectangl，&self等同于所有权，&表示借用，保证其离开作用域而不被内存释放
    // fn area(&mut self) ... 表示可以修改self的值
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 方法名可以和结构体变量名相同
    // 通常，与字段名同名的方法被定义为止返回字段中的值，类似于java中的get/set方法
    fn width(&self) -> bool {
        self.width > 0
    }
    // 带更多参数的方法定义
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数--Self，首字母大写，定义的是一个函数，不是方法（参数不是self）
    // 关联函数经常返回一个结构体新实例的构造函数，类似于java的构造函数
    // Self表示impl后面的类型，这里是Rectangle
    fn square(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let width = rand::thread_rng().gen_range(1..=100);
    let height = rand::thread_rng().gen_range(10..=50);
    let rect = Rectangle { width, height };
    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 60,
        height: 45,
    };
    dbg!(rect.area());
    dbg!(rect1.area());
    // 使用（）表示调用方法，不使用（）表示使用变量
    if rect.width() {
        dbg!(rect.width);
    }
    // 这里rect. rust会根据实际情况，自动添加(&rect.) -- 引用，不可变、(&mut rect.) -- 引用，可变
    dbg!(rect.can_hold(&rect1));
    dbg!(rect.can_hold(&rect2));
    dbg!(rect2.can_hold(&rect1));

    let rect3 = Rectangle::square(50, 50);
    dbg!(rect3.area());
}
