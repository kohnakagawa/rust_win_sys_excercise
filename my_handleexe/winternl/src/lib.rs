#[macro_use(STRUCT, ENUM)]
extern crate winapi;

pub mod nt_system;

#[cfg(test)]
mod tests {
    use nt_system::SYSTEM_HANDLE_HOLDER;
    use nt_system::SYSTEM_HANDLE_INFORMATION;
    use nt_system::SYSTEM_HANDLE;

    #[test]
    fn it_works() {
        let mut sys_handle_holder = SYSTEM_HANDLE_HOLDER::new();
        let _ = match sys_handle_holder.query_system_handle() {
            Ok(_) => {},
            Err(msg) => {println!("{}", msg)},
        };
        let sys_handle_info = sys_handle_holder.raw_ptr();

        unsafe {
            let sys_handle_info: &SYSTEM_HANDLE_INFORMATION
                = &*sys_handle_info;
            let handle_cnt = sys_handle_info.HandleCount;
            let handles = &sys_handle_info.Handles as *const SYSTEM_HANDLE;
            println!("handle cnt = {}", handle_cnt);
            for i in 0..handle_cnt {
                let handle = handles.offset(i as isize);
                let proc_id = (*handle).ProcessId;
                println!("pid = {:?}", proc_id);
            }
        }
    }

}
