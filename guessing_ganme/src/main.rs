// 引入必要的包
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");
    // 创建一个随机数变量
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_number}");
    // 无限循环
    loop {
        println!("please input your number");
        // 定义一个可变变量，该变量是字符串类型
        let mut guess = String::new();
        // 接受用户的输入，并将输入值赋值给guess变量，并添加异常判断expect
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to reade line");
        // 定义一个新的不可变变量（覆盖上面的可变变量），并将上面可变变量mut guess去空格trim，强转成u32类型，并抓取异常
        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");
        // 值比对
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
