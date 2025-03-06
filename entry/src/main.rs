// 引入 libexample 模块中的函数
extern "C" {
    fn print_message();
}

fn main() {
    // 调用 libexample 中的函数
    unsafe {
        print_message();
    }
    println!("This is a message from entry main.");
}