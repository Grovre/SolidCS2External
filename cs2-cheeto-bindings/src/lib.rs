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
    use windows::Win32::System::Threading::{GetCurrentProcessId, OpenProcess, PROCESS_ALL_ACCESS};

    unsafe fn get_this_process_handle() -> windows::core::Result<HANDLE> {
        let pid = GetCurrentProcessId();
        OpenProcess(PROCESS_ALL_ACCESS, false, pid)
    }
    
    #[test]
    fn self_rpm() {
        unsafe {
            let h_proc = get_this_process_handle().unwrap();

            let x = 12345;
            let mut y = 0;

            println!("x: {x}\ny: {y}");

            ReadProcessMemory(
                h_proc,
                &x as *const i32 as *const c_void,
                &mut y as *mut i32 as *mut c_void,
                size_of::<i32>(),
                None)
                .unwrap();

            println!("y after reading to: {y}\n");
            CloseHandle(h_proc).unwrap();
        }
    }

    #[test] 
    fn self_wpm() {
        unsafe {
            let h_proc = get_this_process_handle().unwrap();

            let x = 54321;
            let mut y = 0;

            println!("x: {x}\ny: {y}");

            WriteProcessMemory(
                h_proc, 
                &mut y as *const i32 as *mut c_void, 
                &x as *const i32 as *const c_void, 
                size_of::<i32>(), 
                None)
                .unwrap();

            println!("y after writing to: {y}\n");
            CloseHandle(h_proc).unwrap()
        }
    }
}
