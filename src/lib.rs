#![no_std]

use uefi::table::boot::MemoryDescriptor;

/// information that bootloader passes to the kernel
#[repr(C)]
pub struct BootInfo<'a, 'b> {
    pub memory_map: &'a mut dyn ExactSizeIterator<Item=&'b MemoryDescriptor>,
    /// The offset where the physical memory is mapped at in the virtual address space.
    pub physical_memory_offset: u64,
}
