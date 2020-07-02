




use printf::printf;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::c_int;
use std::ffi::{
    CString
};
use std::fs;
use std::io::{
    Error,
    Write
};
use std::io::prelude::*;
/// ファイルからbytesを読み込んでいく。
fn get_file_resource (path: &String) -> Vec<u8> {
    let mut read_bytes : Vec<u8> = Vec::new();
    let f : Result<std::fs::File, Error> = fs::File::open(path);
    if f.is_ok() != true {
        /// panicせずにエラーのみを表示
        println!("Err {}", f.unwrap_err());
        return read_bytes;
    }
    let f = f.unwrap();
    let byte_list = f.bytes();
    for value in byte_list {
        let t = value.unwrap();
        println!(" - {} - ", &t);
        read_bytes.push(t);
    }
    return read_bytes;
}

fn main() {



    printf_c_string(get_file_resource(&"shift_jis.dat".to_string()));
    return;
    println!("Hello, world!");

    let s_01: String = String :: from ("あいうえお");

    println!("{:?}", s_01.as_bytes().iter());
    for value in s_01.as_bytes().iter() {
        println!("{:?}", value);
    }
    println!("{:?}", s_01.as_bytes().iter().enumerate());
    for (index, value)  in s_01.as_bytes().iter().enumerate() {
        print!("{:?} |", index);
        print!("{:?}", value);
        println!("=================>");
    }
    let key_value = s_01.as_bytes().iter().enumerate();
    println!("{:?}", key_value);

    println!("{}", s_01);
    for (index, value) in key_value
    {

    }
    let text_path : String = "shift_jis.dat".to_string();
    // let read_text_data = fs::read_to_string(&text_path).unwrap();
    unsafe {
        extern "C" {
            fn puts(s: *const c_char) -> c_int;
        }
        let mut f = fs::read("shift_jis.dat").unwrap();
        let mut utf = fs::read_to_string("utf-8.dat").unwrap().as_bytes().to_vec();
        println!("<<<<{:?}>>>>", f);
        println!("unsafe==================================================");
        // Rust上の文字列から std::vec::Vec<u8>構造体を取得
        let percent_s = "%s".to_string().as_bytes().to_vec();
        print_c_string(f);
        print_c_string(utf);
        print_c_string(percent_s.clone());
        let c_string : *const c_char = CString :: new (percent_s).unwrap().as_ptr();


        let void_pointer = CString:: new ("32473048302583284932aiueo".to_string().as_bytes().to_vec()).unwrap().as_ptr() as *mut std::os::raw::c_void;
        println!("{:?}", void_pointer);
        print!("{}", printf(c_string, void_pointer));
        println!("==================================================unsafe");
    }


    let mut my_vector = s_01.as_bytes().to_vec();
    println!("{:?}", my_vector);
    println!("{:?}", my_vector.remove(3));
    println!("{:?}", my_vector);
}
pub
fn print_c_string(output :Vec<u8>) -> isize {
    unsafe {
        extern "C" {
            fn puts(s: *const c_char) -> c_int;
        }
        // Vectorのサイズを取得
        let output_size: isize = output.len() as isize;

        // VectorからCStringを生成
        let to_print = CString::new(output);
        // check_type(&to_print);

        // 無事にCStringを取り出せたとき
        if (to_print.is_ok() == true) {
            puts(to_print.unwrap().as_ptr());
            return output_size;
        } else {
            panic!("{}", to_print.unwrap_err())
        }
        return -1;
    }
}

pub fn printf_c_string(output: Vec<u8>) -> isize {
    unsafe {
        #[link(name="legacy_stdio_definitions", kind="static")]
        extern "C" {
            fn printf(format: *const c_char, args: *mut c_void) -> c_int;
        }

        let c_percent = CString::new("%s".to_string()).unwrap();
        let c_percent_ptr = c_percent.as_ptr() as *const c_char;

        let c_string = CString::new(output).unwrap();
        let c_string_ptr = c_string.as_ptr() as *mut c_void;

        printf(c_percent_ptr, c_string_ptr);
    }
    return -1;
}