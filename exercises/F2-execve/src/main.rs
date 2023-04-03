use std::ffi::CString;

// Argument forwarding
//
// see README for instructions

fn main() {
    let mut path = std::env::current_exe().unwrap();
    path.set_file_name("log");

    let executable = CString::new(path.to_str().unwrap()).unwrap();

    let argv: Vec<_> = std::env::args().map(|s| CString::new(s).unwrap()).collect();
    let mut argv: Vec<_> = argv.iter().map(|s| s.as_ptr()).collect();
    argv.push(std::ptr::null());

    let envp: Vec<_> = std::env::vars()
        .map(|(k, v)| CString::new(format!("{k}={v}")).unwrap())
        .collect();
    let mut envp: Vec<_> = envp.iter().map(|s| s.as_ptr()).collect();
    envp.push(std::ptr::null());

    unsafe {
        libc::execve(
            executable.as_ptr(),
            argv.as_ptr().cast(),
            envp.as_ptr().cast(),
        )
    };

    // if control flow ever gets here, the execve call failed.
    println!("{:#?}", std::io::Error::last_os_error());
}
