extern crate winapi;
use winapi::shared::minwindef::{MAX_PATH, FALSE};
use winapi::um::winnt::{HANDLE};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::tlhelp32::*;

use std::ptr::null_mut;
use std::ffi::CStr;
use std::error;

fn make_module_entry() -> MODULEENTRY32 {
    MODULEENTRY32 {
        dwSize: 0,
        th32ModuleID: 0,
        th32ProcessID: 0,
        GlblcntUsage: 0,
        ProccntUsage: 0,
        modBaseAddr: null_mut(),
        modBaseSize: 0,
        hModule: null_mut(),
        szModule: [0; MAX_MODULE_NAME32 + 1],
        szExePath: [0; MAX_PATH],
    }
}

fn make_process_entry() -> PROCESSENTRY32 {
    PROCESSENTRY32 {
        dwSize: 0,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; MAX_PATH],
    }
}

fn make_snapshot(dw_flags: u32, th32_process_id: u32) -> Result<HANDLE, std::io::Error>{
    match unsafe {CreateToolhelp32Snapshot(dw_flags, th32_process_id)} {
        INVALID_HANDLE_VALUE => {Err(std::io::Error::last_os_error())},
        handle => {Ok(handle)},
    }
}

fn delete_snapshot(handle: HANDLE) {
    unsafe {CloseHandle(handle)};
}

type Pid = u32;

fn make_process_list() -> Result<Vec<(Pid, String)>, Box<error::Error>> {
    let h_snapshot = make_snapshot(TH32CS_SNAPPROCESS, 0)?;
    let mut proc_entry = make_process_entry();
    proc_entry.dwSize = std::mem::size_of_val(&proc_entry) as u32;

    let mut proc_list: Vec<(Pid, String)> = Vec::new();
    unsafe {
        Process32First(h_snapshot, &mut proc_entry);
        loop {
            let proc_name = CStr::from_ptr(&proc_entry.szExeFile[0]).to_str()?.to_string();
            let pid = proc_entry.th32ProcessID;
            proc_list.push((pid, proc_name));
            if Process32Next(h_snapshot, &mut proc_entry) == FALSE { break; }
        }
    }

    delete_snapshot(h_snapshot);
    Ok(proc_list)
}

fn make_module_list(pid: Pid) -> Result<Vec<String>, Box<error::Error>> {
    let h_snapshot = make_snapshot(TH32CS_SNAPMODULE, pid)?;
    let mut mod_entry = make_module_entry();
    mod_entry.dwSize = std::mem::size_of_val(&mod_entry) as u32;

    let mut mod_list: Vec<String> = Vec::new();
    unsafe {
        Module32First(h_snapshot, &mut mod_entry);
        loop {
            if Module32Next(h_snapshot, &mut mod_entry) == FALSE { break; }
            let mod_name = CStr::from_ptr(&mod_entry.szModule[0]).to_str()?.to_string();
            mod_list.push(mod_name);
        }
    }

    delete_snapshot(h_snapshot);
    Ok(mod_list)
}

fn my_tasklist() -> Result<i32, Box<error::Error>> {
    let proc_list = make_process_list()?;
    for process in proc_list {
        println!("pid = {}, name = {}", process.0, process.1);

        if process.0 == 0 {
            println!("\tN/A");
            continue;
        }

        let mod_list = match make_module_list(process.0) {
            Ok(mod_list) => {mod_list},
            Err(_) => {
                println!("\tN/A");
                continue;
            },
        };

        for module in mod_list {
            println!("\t{}", module);
        }
    }
    Ok(0)
}

fn main() -> () {
    let _ = my_tasklist();
}
