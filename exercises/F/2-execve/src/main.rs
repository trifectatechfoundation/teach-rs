use std::ffi::CString;

// Argument forwarding
//
// see README for instructions

fn main() {
    let mut path = std::env::current_exe().unwrap();
    path.set_file_name("log");

    let executable = CString::new(path.to_str().unwrap()).unwrap();

    unsafe { libc::execve(executable.as_ptr(), std::ptr::null(), std::ptr::null()) };

    // if control flow ever gets here, the execve call failed.
    println!("{:#?}", std::io::Error::last_os_error());
}
