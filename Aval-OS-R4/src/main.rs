// main.rs

#![no_std]
#![no_main]

#[macro_use]
extern crate uefi;
extern crate uefi_services;

mod kernel;
mod process;
mod heap;
mod scheduler;


use uefi::prelude::*;
use scheduler::Scheduler;

#[entry]
fn efi_main(image: Handle, st: SystemTable<Boot>) 
    -> Status 
    {
        let mut scheduler
            = Scheduler::new();

        let process 
            = process::Process::new(1, String::from("init"));

        println!("Process: {:?}", process);

        let mut heap 
            = heap::Heap::new(NonNull::dangling(), 1024);

        let ptr
            = heap
                .allocate(32, 4)
                .unwrap();
        loop 
        {
            kernel::kernel_main();
            // Your kernel code goes here
        }
    Status::SUCCESS
}
