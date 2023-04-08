use std::ffi::c_void;

#[repr(C)]
pub struct Numbers(*mut c_void);

#[repr(C)]
pub enum Action {
    Num(usize),
    Append(Numbers, u32),
    Print(Numbers),
    Clear,
    Exit
}
