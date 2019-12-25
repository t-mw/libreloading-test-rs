mod interface;

#[no_mangle]
pub static RELOAD_API: interface::ReloadApi = interface::ReloadApi {
    panic: || {
        panic!("!!!");
    },
};
