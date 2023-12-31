#![feature(core_intrinsics)]

mod util;
mod color;

fn main() {
    dbg!(color::ColorRGB::from_hex(0xacff04));
    dbg!(color::ColorRGB::from_hex(0xacff04).linear());
    dbg!(color::ColorRGB::from_hex(0xacff04).xyz());
}
