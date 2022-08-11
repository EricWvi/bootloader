#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use log::info;
use uefi::prelude::*;

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

    Status::SUCCESS
}
