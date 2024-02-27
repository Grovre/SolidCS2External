use nalgebra::Vector3;

pub extern "C" fn aim_at(
    origin: Vector3<f32>,
    view_offset: Vector3<f32>,
    view_angles: Vector3<f32>,
    target: Vector3<f32>,
    smoothing_factor: f32,
) -> Vector3<f32> {
    let my_pos = origin + view_offset;
    let dv = target - my_pos;
    let dv_len = f32::sqrt(dv.x * dv.x + dv.y * dv.y + dv.z * dv.z);

    let pitch = -f32::asin(dv.z / dv_len) * (180f32 / f32::PI());
    if pitch < -89f32 || pitch > 89f32 {
        return Vector3::new(0f32, 0f32, 0f32);
    }

    let yaw = f32::atan2(dv.y, dv.x) * (180f32 / f32::PI());
    if yaw < -180f32 || yaw > 180f32 {
        return Vector3::new(0f32, 0f32, 0f32);
    }

    let new_pitch = view_angles.x + smoothing_factor * (pitch - view_angles.x);
    let new_yaw = view_angles.y + smoothing_factor * (yaw - view_angles.y);
    let new_angles = Vector3::new(new_pitch, new_yaw, 0f32);

    new_angles
}

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
                None,
            )
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
                None,
            )
            .unwrap();

            println!("y after writing to: {y}\n");
            CloseHandle(h_proc).unwrap()
        }
    }
}
