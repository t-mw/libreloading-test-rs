use libloading::Library;
use std::path::Path;

mod interface;

fn main() {
    let path = std::env::current_dir()
        .unwrap()
        .join("target/debug/libreloadable.dylib"); // unknown backtrace

    // let path = "libreloadable.dylib"; // complete backtrace

    let lib = Library::new(path).unwrap();
    let api = unsafe { lib.get::<*mut interface::ReloadApi>(b"RELOAD_API").unwrap() };

    (unsafe { &**api }.panic)();
}
