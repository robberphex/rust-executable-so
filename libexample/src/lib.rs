use std::os::raw::c_uchar;
use std::process;

// https://www.hustyx.com/cpp/89/
#[unsafe(no_mangle)]
#[unsafe(link_section = ".interp")]
pub static INTERP: [c_uchar;28] = *b"/lib64/ld-linux-x86-64.so.2\0";

#[no_mangle]
pub extern "C" fn _start(){
    print_message();
    println!("This is a message from libexample.so's _start");
    process::exit(1);
}

// 定义一个可以被 C 调用的函数，用于输出信息
#[no_mangle]
pub extern "C" fn print_message() {
    println!("This is a message from libexample.so.");
}
