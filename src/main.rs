use libloading::Library;
use std::path::Path;

mod interface;

fn main() {
    // let path = "libreloadable.dylib"; // complete backtrace
    let path = "target/debug/libreloadable.dylib"; // unknown backtrace

    let lib = Library::new(path).unwrap();
    let api = unsafe { lib.get::<*mut interface::ReloadApi>(b"RELOAD_API").unwrap() };

    (unsafe { &**api }.panic)();
}
