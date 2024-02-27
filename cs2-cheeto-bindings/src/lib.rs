pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// TODO: Make basic RPM/WPM tests
#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::mem::size_of;
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
    use windows::Win32::System::Threading;
    use windows::Win32::System::Threading::{GetCurrentProcessId, OpenProcess};

    unsafe fn get_this_process_handle() -> windows::core::Result<HANDLE> {
        let pid = GetCurrentProcessId();
        OpenProcess(Threading::PROCESS_ALL_ACCESS, false, pid)
    }
    
    #[test]
    fn self_rpm() {
        unsafe {
            let h_proc = get_this_process_handle().unwrap();

            let n = 0;
            let p_n = &n as *const _;
            let buffer = 0;
            let p_buf = &buffer as *const _;

            println!("n: {n}\nbuffer: {buffer}");

            ReadProcessMemory(h_proc, p_n as *const c_void, p_buf as *mut c_void, size_of::<i32>(), None)
                .unwrap();

            println!("buffer after reading from n ({n}): {buffer}\n");
            CloseHandle(h_proc).unwrap();
        }
    }

    #[test] 
    fn self_wpm() {
        unsafe {
            let h_proc = get_this_process_handle().unwrap();

            let n = 0;
            let p_n = &n as *const _;
            let buffer = 1_000_000;
            let p_buf = &buffer as *const _;

            println!("n: {n}\nbuffer: {buffer}");

            let mut bytes_written = usize::default();
            WriteProcessMemory(h_proc, p_n as *mut c_void, p_buf as *const c_void, size_of::<i32>(), Option::Some(&mut bytes_written as *mut usize))
                .unwrap();

            println!("buffer after writing from n ({n}): {buffer}\n");
            CloseHandle(h_proc).unwrap()
        }
    }
}
