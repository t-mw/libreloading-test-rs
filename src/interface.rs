#[repr(C)]
pub struct ReloadApi {
    pub panic: fn(),
}
