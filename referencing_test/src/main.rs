fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("s = {}", s);
    println!("len = {}", len);
    let mut str = String::from("test");
    // change(&s);
    change_mut(&mut str);
    println!("str = {}", str);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(",world");
// }

fn change_mut(some_string: &mut String) {
    some_string.push_str(",hello");
}
