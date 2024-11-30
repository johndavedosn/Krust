#![no_std]
#![no_main]
extern crate alloc;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use my_kernel::task::{executor::Executor, Task};
use my_kernel::{allocator, memory, println};
use x86_64::VirtAddr;
use my_kernel::fs::FileDescriptor;
use my_kernel::OPEN_FILES;
use alloc::string::String;
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
    let mut fd1 = FileDescriptor::new(1);
    let content = String::from("Hello this is text");
    fd1.set_content(content);
    OPEN_FILES
        .lock()
        .open_file(fd1);
    let mut fd2 = FileDescriptor::new(2);
    let content = String::from("This is text too");
    fd2.set_content(content);
    OPEN_FILES
        .lock()
        .open_file(fd2);
    let lock = OPEN_FILES.lock();
    let fd1_content = lock
        .get_file_by_id(1)
        .expect("Failed to find file");
    let fd2_content = lock
        .get_file_by_id(2)
        .expect("Failed to find fine");
    println!("{:?}", fd1_content);
    println!("{:?}", fd2_content);
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
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
