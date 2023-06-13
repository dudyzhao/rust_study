pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

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

pub mod eat_at_restaurant (){
    // 绝对路径--crate表示绝对路径的开始
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径--以模块名开头意味着该路径是相对路径。
    front_of_house::hosting::add_to_waitlist();
}
