extern crate winternl;

use winternl::nt_system::SYSTEM_HANDLE_HOLDER;
use winternl::nt_system::SYSTEM_HANDLE_INFORMATION;
use winternl::nt_system::SYSTEM_HANDLE;

fn main() {
    let mut sys_handle_holder = SYSTEM_HANDLE_HOLDER::new();
    sys_handle_holder.query_system_handle();
    let sys_handle_info = sys_handle_holder.raw_ptr();

    unsafe {
        let sys_handle_info: &SYSTEM_HANDLE_INFORMATION
            = &*sys_handle_info;
        let handle_cnt = sys_handle_info.HandleCount;
        let handles = &sys_handle_info.Handles as *const SYSTEM_HANDLE;

        for i in 0..handle_cnt {
            let handle = handles.offset(i as isize);
            let proc_id = (*handle).ProcessId;
            println!("pid = {}", proc_id);
        }
    }
}
