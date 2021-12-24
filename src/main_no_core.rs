#![no_main]
#![no_std]
#![feature(no_core)]
#![no_core]

#![feature(lang_items)]
#[lang = "sized"]
trait Sized {}

#[no_mangle] 
fn main() -> ! {
    loop {}
}
