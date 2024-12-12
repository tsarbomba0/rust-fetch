use std::mem::MaybeUninit;
use std::ffi::CString;
use std::arch::asm;

// function to return a Result with the struct as Ok and a usize as the Error
pub fn fs_info(path: &str)->Result<libc::statfs, i16>{
    // gets memory to fill out our struct
    let mut info = MaybeUninit::<libc::statfs>::uninit();
    let info_ptr = info.as_mut_ptr();
    // c_string holds the CString made from the path specified
    let c_string: std::ffi::CString;
    match path {
        "/" => c_string = CString::new("/proc").unwrap(),
        _ => c_string = CString::new(path).unwrap(),
    } 
    
    // Pointer to the CString for the file path
    let path_ptr: *const libc::c_char = c_string.as_ptr();
    // Runs the syscall, captures the errno in a variable
    let mut errno: i16;
    unsafe {
        asm!(
            "mov rax,   137",
            "mov rdi,   {ptr}",
            "mov rsi,   {inf}",

            "syscall",

            out("rax") errno, 
            ptr = in(reg) path_ptr,
            inf = in(reg) info_ptr,
        )
    };
    if errno != 0 {
        Err(-errno)
    } else {
        Ok(unsafe{*info_ptr})
    }
}

pub fn match_fs(ftype: u32)->String{
    let fstype = match ftype {
        0xadf5 => "adfs",
        0x5346544e => "ntfs",
        0xef53 => "ext4",
        0x9123683e => "btrfs",
        _ => "unknown"
    };
    fstype.to_string()
}
