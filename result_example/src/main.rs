use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(ref file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    // let _greeting_file = match greeting_file_result {
    //     Ok(f) => f,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {:?}", other_error);
    //         }
    //     },
    // };
    // 上述写法的简写方式 unwrap_or_else
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file :{:?}", error);
            })
        } else {
            panic!("Problem opening the file:{:?}", error);
        }
    });
    print!("the opened file is {:#?}", greeting_file);
    // painc! 的另外2中写法,unwarp()，expect()
    let greeting_file_expect =
        File::open("hello1.txt").expect("hello1.txt should be include in this project");
    print!("unwrap : {:#?}", greeting_file_expect);
    let greeting_file_unwarap = File::open("hello1.txt").unwrap();
    print!("unwrap : {:#?}", greeting_file_unwarap);
}
