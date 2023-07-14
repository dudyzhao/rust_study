fn main() {
    let _vec: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    // 使用 & 和 [] 会得到一个索引位置元素的引用
    // 越界程序会崩溃
    let first: &i32 = &v1[0];
    println!("first is {}", first);
    // 当使用索引作为参数调用 get 方法时，会得到一个可以用于 match 的 Option<&T>
    // 越界会返回None
    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no element."),
    }
    // 循环打印Vec中的所有元素
    for i in &_v {
        println! {"{i}"}
    }

    let mut v = vec![100, 200, 300];
    for i in &mut v {
        *i += 50;
        println! {"now is {i}"};
    }
}
