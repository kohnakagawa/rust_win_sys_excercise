extern crate winapi;

use std::env;
use std::iter::once;
use std::ptr::null_mut;
use std::path::Path;
use winapi::um::processenv::{SetCurrentDirectoryW};
use winapi::um::fileapi::{CreateDirectoryW};

fn my_make_dir(path_name: &str) -> Result<i32, std::io::Error> {
    let mut path_name_buf = path_name.to_owned();
    if path_name_buf.is_empty() {
        return Ok(0);
    }

    if path_name_buf.chars().last() != Some('\\') {
        path_name_buf.push('\\');
    }

    let delim_first = path_name_buf.find('\\').unwrap();
    let dir_parent  = &path_name_buf[..delim_first + 1];
    let dir_child   = &path_name_buf[delim_first + 1..];

    let dir_parent_wide: Vec<u16>
        = dir_parent.encode_utf16().chain(once(0)).collect();

    if !Path::new(dir_parent).exists() {
        println!("{} Directory crated", dir_parent);
        match unsafe { CreateDirectoryW(dir_parent_wide.as_ptr(), null_mut()) } {
            0 => { return Err(std::io::Error::last_os_error()); },
            _ => {},
        };
    }

    match unsafe { SetCurrentDirectoryW(dir_parent_wide.as_ptr()) } {
        0 => { return Err(std::io::Error::last_os_error()); },
        _ => {},
    }

    my_make_dir(dir_child)
}

fn print_usage(prog_name: &str) -> () {
    println!("Usage:");
    println!("{} path_name", prog_name);
}

fn main() -> () {
    let argc = env::args().count();
    let argv : Vec<String> = env::args().collect();
    if argc != 2 {
        print_usage(&argv[0]);
        return;
    }

    if argv[1].is_empty() {
        print_usage(&argv[0]);
        return;
    }

    match my_make_dir(&argv[1]) {
        Ok(_) => println!("create dir {}", argv[1]),
        Err(err) => println!("error occurs: {}", err),
    }
}