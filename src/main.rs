#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
extern crate libc;

use std::ffi::{c_int, CString};

extern "C" {
    // fn cpp_print();
    fn my_c_add(a: c_int, b: c_int) -> c_int;
    fn print_pi();
    fn hello_cpp();
}
fn main() {
    println!(env!("OUT_DIR"));
    unsafe {
        let r = my_c_add(1, 1);
        println!("{}", r);
        print_pi();
        hello_cpp();
        let b: Vec<i8> = CString::new("hamusutaa").unwrap().as_bytes_with_nul().iter().map(|x| *x as i8).collect();
        print_human(Human {
            a: 10,
            name: b.try_into().unwrap()
        });
        print_car(Car {
            a: 520
        });
    }   
}
