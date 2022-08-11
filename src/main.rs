#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec::Vec;
use core::ops::DerefMut;
use log::info;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileHandle, FileMode, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::CStr16;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("bootloader starts working...");
    let bs = system_table.boot_services();
    info!("boot_services address: {:p}", bs);
    let a = 20;
    info!("local variable a address: {:p}", &a);
    bs.stall(10_000_000);

    // You must arrange for the `disable` method to be called or for this logger
    // to be otherwise discarded before boot services are exited.

    // log 截图放到 wiki 里面
    Status::SUCCESS
}

// readelf

fn open_file(bs: &BootServices, path: &str) -> RegularFile {
    info!("opening file: {}", path);
    // TODO: 测试不加 fs 报错情况
    let fs = bs
        .locate_protocol::<SimpleFileSystem>()
        .expect("failed to get SimpleFileSystem");
    let fs = unsafe { &mut *fs.get() };
    let mut root = match fs.open_volume() {
        Err(e) => panic!("{:?}", e),
        Ok(dir) => dir,
    };
    // let mut buf = Vec::with_capacity(path.len());
    // TODO: 查看报错
    let mut buf = [0; 2];
    let path_cstr = match CStr16::from_str_with_buf(path, &mut buf) {
        Err(e) => panic!("{:?}", e),
        Ok(str) => str,
    };
    // TODO: 写一个不存在的文件查看报错
    match root.open(path_cstr, FileMode::Read, FileAttribute::empty()) {
        Ok(handle) => unsafe { RegularFile::new(handle) },
        Err(e) => panic!("{:?}", e),
    }
}
