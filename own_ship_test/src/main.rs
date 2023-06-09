fn main() {
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里
    println!("{}", s); // 这里的s已经被移除，无法再次使用
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("{}", x);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
