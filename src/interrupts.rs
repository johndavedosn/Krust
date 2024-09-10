extern crate alloc;
use crate::hlt_loop;
use crate::{gdt, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use alloc::{vec::Vec};
use crate::io;
use crate::io::Stdin;
    pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const SYSCALL_INTERRUPT: u8 = 0x80;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()]
        .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
        .set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[SYSCALL_INTERRUPT as usize]
        .set_handler_fn(syscall_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {

    x86_64::instructions::interrupts::int3();
}
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, 
}
impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
use spin::Mutex;

lazy_static! {
    static ref STDIN:  Mutex<Stdin> = Mutex::new(io::stdin_new(Vec::new()));
}
extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame
) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode); // new

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}
#[derive(Debug)]
pub struct State {
    rax:   usize,
    rcx:  usize,
    rdx:  usize,
    rbx: usize
}
impl State {
    pub fn new() -> Self {
        State {
            rax:  0,
            rcx:  0,
            rdx:  0,
            rbx: 0
        }
    }
    pub fn update_rax(&mut self){
        let rax: usize;
        unsafe { core::arch::asm!("mov {}, rax", out(reg) rax) }
        self.rax = rax;
    }
    pub fn update_rcx(&mut self) {
        let rcx: usize;
        unsafe { core::arch::asm!("mov {}, rcx", out(reg) rcx) }
        self.rcx = rcx;
    }
    pub fn update_rdx(&mut self) {
        let rdx: usize;
        unsafe { core::arch::asm!("mov {}, rdx", out(reg) rdx) }
        self.rdx = rdx;
    }

    pub fn update_rbx(&mut self) {
        let rbx: usize;
        unsafe { core::arch::asm!("mov {}, rbx", out(reg) rbx) }
        self.rbx = rbx;
    }
}
extern  "x86-interrupt" fn syscall_handler(
    stack_frame: InterruptStackFrame
)
{
    use crate::memory;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;
    use crate::BOOT_INFO;
    let mut regs = State::new();
    match regs.rax {
        1 => {
            regs.update_rax();
            regs.update_rcx();
            regs.update_rdx();
            regs.update_rbx();
            let page = Page::containing_address(VirtAddr::new(regs.rdx as u64));
            let mapper = BOOT_INFO.unwrap().lock();
        }
        _ => {
            unimplemented!()
        }
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(SYSCALL_INTERRUPT);
    }
}