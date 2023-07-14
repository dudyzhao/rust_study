mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_talbe() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// 我们更倾向于使用绝对路径
pub mod eat_at_restaurant (){
    // 绝对路径--crate表示绝对路径的开始
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径--以模块名开头意味着该路径是相对路径。
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order(){}
mod back_of_house {
    fn fix_incorrent_order(){
        cook_order();
        // super使用相对路径，调用与父类同级的数据
        super::deliver_order();
    }
    // 枚举为公有，则其属性也是公有
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn cook_order(){}
    // 公共结构体，但内部字段依然是私有
    pub struct Breakfast {
        // 公共某个字段
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &String) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

