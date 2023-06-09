fn main() {
    let user = User {
        // 创建User结构体实例
        activer: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_account: 1,
    };
    user.email = String::from("anotheremail@example.com"); // 报错，如果像修改struct的值，需要定义 let mut user1 = User{}
}
// 定义一个返回给定email+username 的 struct实例的方法
fn build_user(email: String, username: String) -> User {
    User {
        activer: true,
        username: username,
        email: email,
        sign_in_account: 1,
    }
}

struct User {
    activer: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}
