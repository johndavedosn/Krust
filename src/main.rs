#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_kernel::task::{executor::Executor, Task};
use my_kernel::{allocator, memory, println};
use x86_64::VirtAddr;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use my_kernel::task::keyboard;

    println!("Hello World{}", "!");
    my_kernel::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    // Put kernel code here

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[allow(dead_code)]
fn trigger_syscall(syscall_number: u64, rbx: u64, rcx: u64, rdx: u64) {
    unsafe {
        core::arch::asm!(
        "mov rax, {0}",
        "mov rbx, {1}",
        "mov rcx, {2}",
        "mov rdx, {3}",
        "int 0x80",
        in(reg) syscall_number,
        in(reg) rbx,
        in(reg) rcx,
        in(reg) rdx,
        options(nostack, preserves_flags)
        );
    }
}
async fn async_number() -> u32 {
    42
}
async fn example_task() {
    let n = async_number().await;
    println!("async number: {}", n);
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    my_kernel::hlt_loop();
}
